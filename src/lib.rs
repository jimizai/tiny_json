#![recursion_limit = "256"]

use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone)]
pub enum Json {
    Null,
    Number(f64),
    Boolean(bool),
    String(String),
    Object(Box<HashMap<String, Json>>),
    Array(Vec<Json>),
}

#[macro_export]
macro_rules! json {
    (null) => {
        Json::Null
    };
    ([$($elements: tt), *]) => {
        Json::Array(vec![ $(json!($elements)), * ])
    };
    ({ $($key: tt: $value: tt),* }) => {
        Json::Object(Box::new(
            vec![
                $(($key.to_string(), json!($value))), *
            ].into_iter().collect()
        ))
    };
    ($other: tt) => {
        Json::from($other)
    }
}

macro_rules! impl_from_num_for_json {
    ($($t: ident)*) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize f32 f64);

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl From<&str> for Json {
    fn from(s: &str) -> Json {
        Json::String(s.to_string())
    }
}
