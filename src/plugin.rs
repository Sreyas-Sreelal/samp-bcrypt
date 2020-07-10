use log::{error, info};
use samp::amx::AmxIdent;
use samp::exec_public;
use samp::plugin::SampPlugin;
use samp::prelude::*;
use std::collections::LinkedList;
use std::sync::mpsc::{channel, Receiver, Sender};
use threadpool::ThreadPool;

pub struct SampBcrypt {
    pub hashes: LinkedList<String>,
    pub pool: ThreadPool,
    pub hash_sender: Option<Sender<(u32, String, String)>>,
    pub hash_receiver: Option<Receiver<(u32, String, String)>>,
    pub verify_sender: Option<Sender<(u32, String, bool)>>,
    pub verify_receiver: Option<Receiver<(u32, String, bool)>>,
    pub amx_list: Vec<AmxIdent>,
}

impl SampPlugin for SampBcrypt {
    fn on_load(&mut self) {
        info!("Version: 0.2.3");
        let (verify_sender, verify_receiver) = channel();
        self.verify_sender = Some(verify_sender);
        self.verify_receiver = Some(verify_receiver);

        let (hash_sender, hash_receiver) = channel();
        self.hash_sender = Some(hash_sender);
        self.hash_receiver = Some(hash_receiver);
    }

    fn on_amx_load(&mut self, amx: &Amx) {
        self.amx_list.push(amx.ident());
    }

    fn on_amx_unload(&mut self, amx: &Amx) {
        let raw = amx.ident();
        let index = self.amx_list.iter().position(|x| *x == raw).unwrap();
        self.amx_list.remove(index);
    }

    fn process_tick(&mut self) {
        for (playerid, callback, hashed) in self.hash_receiver.as_ref().unwrap().try_iter() {
            let mut executed = false;
            self.hashes.push_front(hashed);
            for amx in &self.amx_list {
                if let Some(amx) = samp::amx::get(*amx) {
                    let _ = exec_public!(amx, &callback, playerid);
                    executed = true;
                }
            }
            if !executed {
                error!("*Cannot execute callback {:?}", callback);
            }
        }

        for (playerid, callback, success) in self.verify_receiver.as_ref().unwrap().try_iter() {
            let mut executed = false;
            for amx in &self.amx_list {
                if let Some(amx) = samp::amx::get(*amx) {
                    let _ = exec_public!(amx, &callback, playerid, success);
                    executed = true;
                }
            }
            if !executed {
                error!("*Cannot execute callback {:?}", callback);
            }
        }

        self.hashes.clear();
    }
}
