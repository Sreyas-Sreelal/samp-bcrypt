#[macro_use]
extern crate samp_sdk;
extern crate bcrypt;

#[macro_use] 
mod macros;
mod plugin;
mod natives;
mod internals;

use plugin::SampBcrypt;

new_plugin!(SampBcrypt with process_tick);

