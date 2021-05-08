use bcrypt::{hash, verify};
use log::error;
use std::sync::mpsc::Sender;
#[derive(Debug)]
pub enum ArgumentTypes {
    Primitive(i32),
    String(Vec<u8>),
}
pub fn hash_verify(
    verify_sender: Option<Sender<(i32, String, bool)>>,
    playerid: i32,
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
    hash_sender: Option<Sender<(i32, String, String, Vec<ArgumentTypes>)>>,
    playerid: i32,
    input: String,
    callback: String,
    cost: u32,
    optional_args: Vec<ArgumentTypes>,
) {
    match hash(&input, cost) {
        Ok(hashed) => {
            let _ = hash_sender
                .as_ref()
                .unwrap()
                .send((playerid, callback, hashed, optional_args));
        }
        Err(err) => {
            error!("{} => {:?}", callback, err);
        }
    }
}
