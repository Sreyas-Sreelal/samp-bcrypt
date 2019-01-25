#[macro_use]
extern crate samp_sdk;
extern crate bcrypt;

#[macro_use]
mod macros;
mod internals;
mod natives;
mod plugin;

use plugin::SampBcrypt;

new_plugin!(SampBcrypt with process_tick);
