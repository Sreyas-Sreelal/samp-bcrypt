use samp::plugin::SampPlugin;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub struct SampBcrypt {
    pub hashes: Arc<Mutex<LinkedList<String>>>,
    pub pool: ThreadPool,
}

impl SampPlugin for SampBcrypt {}
