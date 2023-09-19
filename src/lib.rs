pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/gecode_bindings.rs"));
}

use bindings::GecodeVersion;
