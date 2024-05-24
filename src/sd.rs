use bincode;
use serde::{Deserialize, Serialize};

/// Serialize to binrary
pub fn to_bin<T: Serialize>(o: T) -> Result<Vec<u8>, &'static str> {
    match bincode::serialize(&o) {
        Ok(s) => Ok(s),
        Err(_) => Err("can not serialize bin"),
    }
}

/// Deserialize from binrary
pub fn from_bin<'de, T: Deserialize<'de>>(b: &'de [u8]) -> Result<T, &'static str> {
    match bincode::deserialize(b) {
        Ok(t) => Ok(t),
        Err(_) => Err("can not deserialize bin"),
    }
}

/// Serialize to json string
pub fn to_json<T: Serialize>(o: T) -> Result<String, &'static str> {
    match serde_json::to_string(&o) {
        Ok(s) => Ok(s),
        Err(_) => Err("can not serialize json"),
    }
}

/// Deserialize from json string
pub fn from_json<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, &'static str> {
    match serde_json::from_str(s) {
        Ok(t) => Ok(t),
        Err(_) => Err("can not deserialize json"),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Aoo {
    name: String,
    age: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tjson() {
        let aoo = Aoo {
            name: String::from("ok"),
            age: 18,
        };
        let res = to_json(aoo).unwrap();
        println!("aoo is {}", res);
    }

    #[test]
    fn fjson() {
        let s = r#"{"name":"ok","age":18}"#;
        let aoo = from_json::<'_, Aoo>(s).unwrap();
        println!("Aoo is {:?}", aoo)
    }
}
