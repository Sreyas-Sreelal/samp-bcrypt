use crate::internals::ArgumentTypes;
use crate::internals::*;
use log::error;
use samp::error::AmxError;
use samp::native;
use samp::prelude::*;

impl super::SampBcrypt {
    #[native(raw, name = "bcrypt_hash")]
    pub fn bcrypt_hash(&mut self, amx: &Amx, mut args: samp::args::Args) -> AmxResult<bool> {
        let playerid = args.next::<u32>().ok_or(AmxError::Params)?;
        let callback = args
            .next::<AmxString>()
            .ok_or(AmxError::Params)?
            .to_string();
        let input = args
            .next::<AmxString>()
            .ok_or(AmxError::Params)?
            .to_string();
        let cost = args.next::<u32>().ok_or(AmxError::Params)?;
        let format = args.next::<AmxString>().ok_or(AmxError::Params)?.to_bytes();
        if format.len() != args.count() - 5 {
            error!(
                "The argument count mismatch expected :{} provided: {}.",
                format.len(),
                args.count() - 5
            );
            return Ok(false);
        }
        let sender = self.hash_sender.clone();
        let mut optional_args: Vec<ArgumentTypes> = Vec::new();

        for specifiers in format {
            match specifiers {
                b'd' | b'i' | b'f' => {
                    optional_args.push(ArgumentTypes::Primitive(
                        *args.next::<Ref<i32>>().ok_or(AmxError::Params)?,
                    ));
                }
                b's' => {
                    let argument: Ref<i32> = args.next().ok_or(AmxError::Params)?;
                    let amx_str = AmxString::from_raw(amx, argument.address())?;
                    optional_args.push(ArgumentTypes::String(amx_str.to_bytes()));
                }
                _ => {
                    error!("Unknown specifier type {}", specifiers);
                    return Ok(false);
                }
            }
        }

        self.pool.execute(move || {
            hash_start(sender, playerid, input, callback, cost, optional_args);
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
