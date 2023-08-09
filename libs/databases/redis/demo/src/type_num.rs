use redis::{Commands, Connection, RedisResult};

pub(crate) fn fetch_an_integer(con: &mut Connection) -> RedisResult<isize> {
    // throw away the result, just make sure it does not fail
    con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}
