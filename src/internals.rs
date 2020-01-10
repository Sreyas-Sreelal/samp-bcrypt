use bcrypt::{hash, verify};
use log::error;
use std::sync::mpsc::Sender;

pub fn hash_verify(
    verify_sender: Option<Sender<(u32, String, bool)>>,
    playerid: u32,
    input: String,
    hash: String,
    callback: String,
) {
    match verify(&input, &hash) {
        Ok(success) => {
            let _ = verify_sender
                .as_ref()
                .unwrap()
                .send((playerid, callback, success));
        }
        Err(err) => {
            error!("{} => {:?}", callback, err);
        }
    }
}

pub fn hash_start(
    hash_sender: Option<Sender<(u32, String, String)>>,
    playerid: u32,
    input: String,
    callback: String,
    cost: u32,
) {
    match hash(&input, cost) {
        Ok(hashed) => {
            let _ = hash_sender
                .as_ref()
                .unwrap()
                .send((playerid, callback, hashed));
        }
        Err(err) => {
            error!("{} => {:?}", callback, err);
        }
    }
}
