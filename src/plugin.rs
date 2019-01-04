use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use natives::Natives;
use bcrypt::{hash,verify};
use std::sync::mpsc::{Sender,Receiver,channel};

define_native!(bcrypt_hash,playerid:u32,callback:String,input:String,cost:u32);
define_native!(bcrypt_get_hash,contextid:usize,dest:ref Cell,size:usize);
define_native!(bcrypt_verify,playerid:u32,callback:String,input:String,hash:String);
define_native!(bcrypt_delete,contextid:usize);

pub struct SampBcrypt{
	pub hash_start_sender: Option<Sender<(u32, String,String, u32)>>,
	pub hash_complete_receiver: Option<Receiver<(u32, String,String)>>,
	pub verify_start_sender: Option<Sender<(u32, String,String,String)>>,
	pub verify_complete_receiver: Option<Receiver<(u32,String,bool)>>,
	pub hashes: Vec<String>,
	pub hash_context_id: usize,
	pub amx_list: Vec<usize>,
}

impl SampBcrypt {
	pub fn load(&mut self) -> bool {
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

		let (verify_complete_send, verify_complete_receiver) = channel();
		let (verify_start_sender, verify_start_receiver) = channel();
		
		self.verify_complete_receiver = Some(verify_complete_receiver);
		self.verify_start_sender = Some(verify_start_sender);
		
		std::thread::spawn(move || {
			for (playerid,callback,input,hash) in verify_start_receiver.iter() {
				match verify(&input, &hash){
					Ok(success) =>{
						verify_complete_send.send((playerid,callback,success)).unwrap();
					},
					Err(err)=>{
						verify_complete_send.send((playerid,callback,false)).unwrap();
						log!("**[SampBcrypt] Verify error {:?}",err);
					}
				}
			}	
		});

		log!("**[SampBcrypt] Loaded!");

		return true;
	}

	pub fn process_tick(&mut self) {
		for (playerid, callback, hashed) in  self.hash_complete_receiver.as_ref().unwrap().try_iter() {
			for amx in &self.amx_list{
				let amx = AMX::new(*amx as *mut _);
				self.hashes.push(hashed.clone());
				self.hash_context_id += 1;
				match exec_public_with_name!(amx,callback;playerid,self.hash_context_id -1){
					Ok(_) => {

					},
					Err(err) => {
						log!("Error executing {:?} \n {:?}",callback,err);
					}
				}
			}
		}

		for (playerid,callback,success) in  self.verify_complete_receiver.as_ref().unwrap().try_iter() {
			for amx in &self.amx_list{
				let amx = AMX::new(*amx as *mut _);
				match exec_public_with_name!(amx,callback;playerid,success){
					Ok(_) => {

					},
					Err(err) => {
						log!("Error executing {:?} \n {:?}",callback,err);
					}
				};					
			}
		}
	}

	pub fn unload(&self) {
		log!("**[SampBcrypt] Unloaded!");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		self.amx_list.push(amx.amx as usize);
		let natives = natives!{
			"bcrypt_hash" => bcrypt_hash,
			"bcrypt_get_hash" => bcrypt_get_hash,
			"bcrypt_delete" => bcrypt_delete,
			"bcrypt_verify" => bcrypt_verify
		};

		match amx.register(&natives) {
			Ok(_) => log!("**[SampBcrypt] Natives are successful loaded"),
			Err(err) => log!("**[SampBcrypt] Error loading one of the natives {:?}", err),
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
			hash_start_sender: None,
			hash_complete_receiver: None,
			verify_start_sender: None,
			verify_complete_receiver: None,
			amx_list: Vec::new(),
			hashes: Vec::new(),
			hash_context_id: 0,
		}
	}
}