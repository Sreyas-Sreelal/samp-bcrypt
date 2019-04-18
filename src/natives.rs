use crate::internals::*;
use samp::cell::{AmxString, UnsizedBuffer};
use samp::error::AmxResult;
use samp::prelude::*;
use samp::{native, AmxAsyncExt};

impl super::SampBcrypt {
    #[native(name = "bcrypt_hash")]
    pub fn bcrypt_hash(
        &mut self,
        amx: &Amx,
        playerid: u32,
        callback: AmxString,
        input: AmxString,
        cost: u32,
    ) -> AmxResult<(bool)> {
        let amx = amx.to_async();
        let callback = callback.to_string();
        let input = input.to_string();
        let hashes = self.hashes.clone();

        std::thread::spawn(move || {
            hash_start(amx, playerid, input, callback, cost, hashes);
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
        match self.hashes.lock().unwrap().front() {
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
        amx: &Amx,
        playerid: u32,
        callback: AmxString,
        input: AmxString,
        hash: AmxString,
    ) -> AmxResult<bool> {
        let callback = callback.to_string();
        let input = input.to_string();
        let hash = hash.to_string();
        let amx = amx.to_async();

        std::thread::spawn(move || {
            hash_verify(amx, playerid, input, hash, callback);
        });

        Ok(true)
    }
}
