use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};

pub trait Natives {
	fn bcrypt_hash(&mut self,_:&AMX,playerid:u32,callback:String,input:String,cost:u32) -> AmxResult<Cell>;
}

impl Natives for super::SampBcrypt{
	fn bcrypt_hash(&mut self,_:&AMX,playerid:u32,callback:String,input:String,cost:u32) -> AmxResult<Cell>{
		self.hash_start_sender.as_ref().unwrap().send((playerid,callback,input,cost)).unwrap();
		Ok(1)
	}
}


