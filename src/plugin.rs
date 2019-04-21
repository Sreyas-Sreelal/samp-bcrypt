use log::info;
use samp::plugin::SampPlugin;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub struct SampBcrypt {
    pub hashes: Arc<Mutex<LinkedList<String>>>,
    pub pool: ThreadPool,
}

impl SampPlugin for SampBcrypt {
    fn on_load(&mut self) {
        info!(
            " 
   ###############################################################
   #                      SampBcrypt                             #
   #                        V0.2.0 Loaded!!                      #
   #   Found any bugs? Report it here:                           #
   #       https://github.com/Sreyas-Sreelal/samp-bcrypt/issues  #
   #                                                             #
   ###############################################################
        "
        );
    }

    fn on_unload(self: Box<SampBcrypt>) {
        info!("Unloaded!");
    }
}
