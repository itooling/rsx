use serde::{Deserialize, Serialize};

pub fn to<T: Serialize>(o: T) -> Result<String, &'static str> {
    if let Ok(s) = serde_json::to_string(&o) {
        return Ok(s);
    }
    Err("can not serialize")
}

pub fn from<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, &'static str> {
    if let Ok(t) = serde_json::from_str(s) {
        return Ok(t);
    }
    Err("can not deserialize")
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
        let res = to(aoo).unwrap();
        println!("aoo is {}", res);
    }

    #[test]
    fn fjson() {
        let s = r#"{"name":"ok","age":18}"#;
        let aoo = from::<'_, Aoo>(s).unwrap();
        println!("Aoo is {:?}", aoo)
    }
}
