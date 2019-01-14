#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "bundled")]
extern crate jq_src;

mod bindings;
pub use bindings::*;

#[cfg(test)]
mod tests {
    use super::{jq_init, jq_state, jq_teardown};

    #[test]
    fn can_init_and_teardown() {
        unsafe {
            let state: *mut jq_state = jq_init();
            jq_teardown(state as *mut *mut jq_state);
        }
    }
}
