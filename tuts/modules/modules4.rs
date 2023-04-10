/**
 * This module is all about showing difference b/w `pub`, `pub(crate)`, `pub(super)`
 */

mod house {
    pub(super) fn ring_door_bell() {}
    pub fn get_door_bell_status() {
        // get door bell status
    }

    mod kitchen {}

    mod m_bedroom {

        pub(crate) fn get_entry_status() {
            // see the status for door bell if ringed
            super::get_door_bell_status();
        }

        mod bathroom {}
    }
}

use super::*;

pub fn main() {
    house::ring_door_bell();
    house::get_door_bell_status();
}
