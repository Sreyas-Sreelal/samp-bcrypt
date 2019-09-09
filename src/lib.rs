mod internals;
mod natives;
mod plugin;

use plugin::SampBcrypt;
use samp::initialize_plugin;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

initialize_plugin!(
    natives: [
        SampBcrypt::bcrypt_hash,
        SampBcrypt::bcrypt_get_hash,
        SampBcrypt::bcrypt_verify,
        SampBcrypt::bcrypt_set_thread_limit
    ],
    {
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[SampBcrypt] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .apply();

        SampBcrypt {
            hashes: Arc::new(Mutex::new(LinkedList::new())),
            pool: ThreadPool::new(3)
        }
    }
);
