/* 
    represent time considering sub units like 10s 7ns
*/

use std::time::Duration;

pub fn run() {
    let ten_sec = Duration::from_secs(10);      // 10 sec
    let five_sec = Duration::from_nanos(7);     // 7 nano sec
    let total = ten_sec + five_sec;

    assert_eq!(total, Duration::new(10, 7));
}