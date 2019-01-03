#[macro_use]
extern crate samp_sdk;
extern crate bcrypt;

mod plugin;
mod natives;

use plugin::SampBcrypt;

new_plugin!(SampBcrypt with process_tick);

