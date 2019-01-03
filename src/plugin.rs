use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use natives::Natives;
use bcrypt::{hash};
use std::sync::mpsc::{Sender,Receiver,channel};

define_native!(bcrypt_hash,playerid:u32,callback:String,input:String,cost:u32);

pub struct SampBcrypt{
	pub hash_start_sender: Option<Sender<(u32, String,String, u32)>>,
	pub has_complete_recv: Option<Receiver<(u32, String,String,bool)>>,
	pub amx_list :Vec<usize>,
}

impl SampBcrypt {
	pub fn load(&mut self) -> bool {
		let (send_response, rcv_response) = channel();
		let (hash_start_send, rcv_request) = channel();
		
		self.has_complete_recv = Some(rcv_response);
		self.hash_start_sender = Some(hash_start_send);
			
		std::thread::spawn(move || {
			for (playerid,callback,input,cost) in rcv_request.iter() {
				match hash(&input, cost){
					Ok(hashed) =>{
						send_response.send((playerid,callback,hashed,true)).unwrap();
					},
					Err(err)=>{
						send_response.send((playerid,callback,String::from(""),false)).unwrap();
						log!("**[SampBcrypt] Hash error {:?}",err);
					}
				}
			}	
		});
		log!("SampBcrypt Loaded!");
		return true;
	}

	pub fn process_tick(&mut self) {
		for (playerid, callback, hashed,success) in  self.has_complete_recv.as_ref().unwrap().try_iter() {
			for amx in &self.amx_list{
				let amx = AMX::new(*amx as *mut _);
				match amx.find_public(&callback){
					Ok(index) =>{
						amx.push(success).unwrap();
						amx.push_string(&hashed,false).unwrap();
						amx.push(playerid).unwrap();
						amx.exec(index).unwrap();
					}
					Err(err) =>{
						log!("**[SampBcrypt] Error finding callback {:?}",err);
						continue;
					}
				};
			}
		}
	}

	pub fn unload(&self) {
		log!("SampBcrypt Unloaded!");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		self.amx_list.push(amx.amx as usize);
		let natives = natives!{
			"bcrypt_hash" => bcrypt_hash
		};

		match amx.register(&natives) {
			Ok(_) => log!("Natives are successful loaded"),
			Err(err) => log!("Whoops, there is an error {:?}", err),
		}

		AMX_ERR_NONE
	}

	pub fn amx_unload(&mut self, amx: &mut AMX) -> Cell {
		let raw = amx.amx as usize;
		let index = self.amx_list.iter().position(|x| *x == raw).unwrap().clone();
		self.amx_list.remove(index);
		AMX_ERR_NONE
	}

}

impl Default for SampBcrypt {
	fn default() -> Self {
		SampBcrypt {
			hash_start_sender:None,
			has_complete_recv:None,
			amx_list:Vec::new(),
		}
	}
}