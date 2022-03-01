/* 
    time module
    - units: sec, milli_sec, micro_sec, nano_sec
*/

use std::time::Duration;

pub fn run() {
    let t1 = Duration::from_secs(60);
    let t2 = Duration::from_millis(60_000);
    let t3 = Duration::from_micros(60_000_000);
    let t4 = Duration::from_nanos(60_000_000_000);

    assert_eq!(t1, t2);
    assert_eq!(t1, t3);
    assert_eq!(t1, t4);

}