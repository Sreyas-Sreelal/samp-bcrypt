use bcrypt::{hash,verify};
use std::sync::mpsc::channel;

pub trait Internal {
    fn listen_for_hash_requests(&mut self);
	fn listen_for_verify_requests(&mut self);
}

impl Internal for super::SampBcrypt {
    fn listen_for_hash_requests(&mut self) {
        let (hash_complete_send, hash_complete_receiver) = channel();
		let (hash_start_sender, hash_start_receiver) = channel();
        
        self.hash_complete_receiver = Some(hash_complete_receiver);
		self.hash_start_sender = Some(hash_start_sender);
		
		std::thread::spawn(move || {
			for (playerid,callback,input,cost) in hash_start_receiver.iter() {
				match hash(&input, cost){
					Ok(hashed) =>{
						hash_complete_send.send((playerid,callback,hashed)).unwrap();
					},
					Err(err)=>{
						hash_complete_send.send((playerid,callback,String::from(""))).unwrap();
						log!("**[SampBcrypt] Hash error {:?}",err);
					}
				}
			}	
		});
    }

    fn listen_for_verify_requests(&mut self) {
        let (verify_complete_send, verify_complete_receiver) = channel();
		let (verify_start_sender, verify_start_receiver) = channel();
		
		self.verify_complete_receiver = Some(verify_complete_receiver);
		self.verify_start_sender = Some(verify_start_sender);
		
		std::thread::spawn(move || {
			for (playerid,callback,input,hash) in verify_start_receiver.iter() {
				match verify(&input, &hash){
					Ok(success) => {
						verify_complete_send.send((playerid,callback,success)).unwrap();
					},
					Err(err) => {
						verify_complete_send.send((playerid,callback,false)).unwrap();
						log!("**[SampBcrypt] Verify error {:?}",err);
					}
				}
			}	
		});
    }
}