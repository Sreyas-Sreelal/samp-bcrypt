mod internals;
mod natives;
mod plugin;

use plugin::SampBcrypt;
use samp::initialize_plugin;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

initialize_plugin!(
    natives: [
        SampBcrypt::bcrypt_hash,
        SampBcrypt::bcrypt_get_hash,
        SampBcrypt::bcrypt_verify
    ],
    {
        samp::plugin::enable_process_tick();
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let log_file = fern::log_file("SampBcrypt.log").expect("Cannot create log file!");

        let trace_level = fern::Dispatch::new()
            .level(log::LevelFilter::Trace)
            .chain(log_file);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[SampBcrypt] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .chain(trace_level)
            .apply();

        let plugin = SampBcrypt {
            hashes: Arc::new(Mutex::new(LinkedList::new())),
        };
        plugin
    }
);
