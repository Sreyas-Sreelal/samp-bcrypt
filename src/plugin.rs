use log::info;
use samp::plugin::SampPlugin;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

pub struct SampBcrypt {
    pub hashes: Arc<Mutex<LinkedList<String>>>,
}

impl SampPlugin for SampBcrypt {
    fn on_load(&mut self) {
        info!("Loaded!");
    }

    fn on_unload(self: Box<SampBcrypt>) {
        info!("Unloaded!");
    }
}
