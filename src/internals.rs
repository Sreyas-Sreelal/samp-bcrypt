use bcrypt::{hash, verify};
use std::sync::mpsc::channel;

pub fn listen_for_hash_requests(plugin: &mut super::SampBcrypt) {
    let (hash_complete_send, hash_complete_receiver) = channel();
    let (hash_start_sender, hash_start_receiver) = channel();

    plugin.hash_complete_receiver = Some(hash_complete_receiver);
    plugin.hash_start_sender = Some(hash_start_sender);

    std::thread::spawn(move || {
        for (playerid, callback, input, cost) in hash_start_receiver.iter() {
            match hash(&input, cost) {
                Ok(hashed) => {
                    hash_complete_send
                        .send((playerid, callback, hashed))
                        .unwrap();
                }
                Err(err) => {
                    hash_complete_send
                        .send((playerid, callback, String::from("")))
                        .unwrap();
                    log!("**[SampBcrypt] Hash error {:?}", err);
                }
            }
        }
    });
}

pub fn listen_for_verify_requests(plugin: &mut super::SampBcrypt) {
    let (verify_complete_send, verify_complete_receiver) = channel();
    let (verify_start_sender, verify_start_receiver) = channel();

    plugin.verify_complete_receiver = Some(verify_complete_receiver);
    plugin.verify_start_sender = Some(verify_start_sender);

    std::thread::spawn(move || {
        for (playerid, callback, input, hash) in verify_start_receiver.iter() {
            match verify(&input, &hash) {
                Ok(success) => {
                    verify_complete_send
                        .send((playerid, callback, success))
                        .unwrap();
                }
                Err(err) => {
                    verify_complete_send
                        .send((playerid, callback, false))
                        .unwrap();
                    log!("**[SampBcrypt] Verify error {:?}", err);
                }
            }
        }
    });
}
