#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/action_bindings.rs"));

use rcl::*;
use msg_gen::*;
