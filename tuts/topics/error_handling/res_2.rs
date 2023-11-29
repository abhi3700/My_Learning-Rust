/* 
    Convert Result to Option
*/

pub fn run() {
    let o: Result<u8, String> = Ok(90);
    let e: Result<u8, String> = Err("Wrong value".to_string());

    // apply ok() on both
    // assert_eq!(o.ok(), Some(90));
    // assert_eq!(e.ok(), None);

    // apply err() on both
    assert_eq!(o.err(), None);
    assert_eq!(e.err(), Some("Wrong value".to_string()));

}