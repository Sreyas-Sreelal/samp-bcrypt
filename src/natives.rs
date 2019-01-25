use samp_sdk::amx::{AmxResult, AMX};
use samp_sdk::types::Cell;

impl super::SampBcrypt {
    pub fn bcrypt_hash(
        &mut self,
        _: &AMX,
        playerid: u32,
        callback: String,
        input: String,
        cost: u32,
    ) -> AmxResult<Cell> {
        self.hash_start_sender
            .as_ref()
            .unwrap()
            .send((playerid, callback, input, cost))
            .unwrap();
        Ok(1)
    }

    pub fn bcrypt_get_hash(
        &mut self,
        _: &AMX,
        contextid: usize,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        if contextid > self.hash_context_id {
            log!(
                "**[SampBcrypt] Invalid context id {:?} is passed",
                contextid
            );
            Ok(0)
        } else {
            let hash = &self.hashes[contextid];
            match samp_sdk::cp1251::encode(hash) {
                Ok(hash_encoded) => {
                    set_string!(hash_encoded, dest, size);
                    Ok(1)
                }
                Err(err) => {
                    log!("**[SampBcrypt] Encoding error cannot set hash to destination string \n {:?}",err);
                    Ok(0)
                }
            }
        }
    }

    pub fn bcrypt_delete(&mut self, _: &AMX, contextid: usize) -> AmxResult<Cell> {
        if contextid > self.hash_context_id {
            log!(
                "**[SampBcrypt] Invalid context id {:?} is passed",
                contextid
            );
            Ok(0)
        } else {
            self.hashes.remove(contextid);
            Ok(1)
        }
    }

    pub fn bcrypt_verify(
        &mut self,
        _: &AMX,
        playerid: u32,
        callback: String,
        input: String,
        hash: String,
    ) -> AmxResult<Cell> {
        self.verify_start_sender
            .as_ref()
            .unwrap()
            .send((playerid, callback, input, hash))
            .unwrap();
        Ok(1)
    }
}
