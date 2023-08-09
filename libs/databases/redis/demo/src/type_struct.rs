use chrono::Utc;
use serde::{Deserialize, Serialize};
// couldn't use as it doesn't have serde::Serialize trait implemented.
// Required as struct param when the struct is implemented with Serialize trait.
// Why so?
// - Because when the entire struct calls one of the trait methods, then the struct param
//      becomes a blocker as it doesn't have the trait implemented.
// use std::time::Instant;

use redis::{Commands, Connection, FromRedisValue, RedisResult};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Person {
    id: u16,
    name: String,
    age: u8,
    work: String,
    went_to_work: i64,
}

// This trait implemented because of struct type to be used during `set()` function
impl FromRedisValue for Person {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let s = match v {
            redis::Value::Data(data) => String::from_utf8_lossy(data),
            _ => {
                return Err(redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Invalid Redis value type",
                )))
            }
        };

        serde_json::from_str(&s).map_err(|_| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Failed to deserialize Redis value",
            ))
        })
    }
}

pub(crate) fn fetch_person_struct(con: &mut Connection) -> RedisResult<Person> {
    let p1 = Person {
        id: 1,
        name: "Abhi".to_string(),
        age: 30,
        work: "Developer".to_string(),
        went_to_work: Utc::now().timestamp(),
    };

    // serialize struct
    let p1_serialized = serde_json::to_string(&p1).expect("Failure in serializing Person");

    // set with p1.id as key
    let _ = con.set::<_, _, Person>(&format!("{}", p1.id), &p1_serialized);

    // automatically deserialized
    con.get::<_, Person>(p1.id)
}
