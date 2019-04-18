use bcrypt::{hash, verify};
use log::error;
use samp::{exec_public, AsyncAmx,AmxLockError};
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

pub fn hash_verify(amx: AsyncAmx, playerid: u32, input: String, hash: String, callback: String) {
    match verify(&input, &hash) {
        Ok(success) => {
            let amx = match amx.lock() {
                Err(AmxLockError::AmxGone) => {
                    error!("{} => AMX is gone",callback);
                    return;
                }
                Err(_) => {
                    error!("{} => mutex is poisoned",callback);
                    return;
                }
                Ok(amx) => amx,
            };
            match exec_public!(amx, &callback, playerid, success) {
                Ok(_) => {}
                Err(err) => {
                    error!("Unable to execute {:?} => {:?}", callback, err);
                }
            };
        }
        Err(err) => {
            error!("{} => {:?}", callback,err);
        }
    }
}

pub fn hash_start(
    amx: AsyncAmx,
    playerid: u32,
    input: String,
    callback: String,
    cost: u32,
    hashes: Arc<Mutex<LinkedList<String>>>,
) {
    match hash(&input, cost) {
        Ok(hashed) => {
            let amx = match amx.lock() {
                Ok(amx) => amx,
                Err(AmxLockError::AmxGone) => {
                    error!("{} => AMX is gone",callback);
                    return;
                }
                Err(_) => {
                    error!("{} => mutex is poisoned",callback);
                    return;
                }
            };

            hashes.lock().unwrap().push_front(hashed);

            match exec_public!(amx, &callback, playerid) {
                Ok(_) => {
                }
                Err(err) => {
                    error!("Unable to execute {:?} => {:?}", callback, err);
                }
            }
            hashes.lock().unwrap().clear();
        }

        Err(err) => {
            error!("{} => {:?}", callback,err);
        }
    }
}
