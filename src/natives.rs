use crate::internals::*;
use log::error;
use samp::native;
use samp::prelude::*;

impl super::SampBcrypt {
    #[native(name = "bcrypt_hash")]
    pub fn bcrypt_hash(
        &mut self,
        _: &Amx,
        playerid: u32,
        callback: AmxString,
        input: AmxString,
        cost: u32,
    ) -> AmxResult<bool> {
        let callback = callback.to_string();
        let input = input.to_string();
        let sender = self.hash_sender.clone();
        self.pool.execute(move || {
            hash_start(sender, playerid, input, callback, cost);
        });

        Ok(true)
    }

    #[native(name = "bcrypt_get_hash")]
    pub fn bcrypt_get_hash(
        &mut self,
        _: &Amx,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        match self.hashes.front() {
            Some(hash) => {
                let mut dest = dest.into_sized_buffer(size);
                let _ = samp::cell::string::put_in_buffer(&mut dest, &hash);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    #[native(name = "bcrypt_verify")]
    pub fn bcrypt_verify(
        &mut self,
        _: &Amx,
        playerid: u32,
        callback: AmxString,
        input: AmxString,
        hash: AmxString,
    ) -> AmxResult<bool> {
        let callback = callback.to_string();
        let input = input.to_string();
        let hash = hash.to_string();
        let sender = self.verify_sender.clone();
        self.pool.execute(move || {
            hash_verify(sender, playerid, input, hash, callback);
        });

        Ok(true)
    }

    #[native(name = "bcrypt_set_thread_limit")]
    pub fn bcrypt_set_thread_limit(&mut self, _: &Amx, value: i32) -> AmxResult<bool> {
        if value < 1 {
            error!("Number of threads must be atleast 1");
            return Ok(false);
        }
        self.pool.set_num_threads(value as usize);
        Ok(true)
    }
}
