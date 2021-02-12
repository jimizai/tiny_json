#[macro_use]
extern crate json;
use json::Json;

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn json_array() {
    assert_eq!(
        json!([null, null]),
        Json::Array(vec![Json::Null, Json::Null])
    );
}

#[test]
fn json_complex() {
    assert_eq!(
        json!([
            {
                "name": "Luke",
                "age": 18,
                "major": "Science",
                "is_study": true
            },
            {
                "name": "XiaoMing",
                "class_of": 1902,
                "major": "Knots",
                "is_study": false
            }
        ]),
        Json::Array(vec![
            Json::Object(Box::new(
                vec![
                    ("name".to_string(), Json::String("Luke".to_string())),
                    ("age".to_string(), Json::Number(18 as f64)),
                    ("major".to_string(), Json::String("Science".to_string())),
                    ("is_study".to_string(), Json::Boolean(true)),
                ]
                .into_iter()
                .collect()
            )),
            Json::Object(Box::new(
                vec![
                    ("name".to_string(), Json::String("XiaoMing".to_string())),
                    ("class_of".to_string(), Json::Number(1902 as f64)),
                    ("major".to_string(), Json::String("Knots".to_string())),
                    ("is_study".to_string(), Json::Boolean(false)),
                ]
                .into_iter()
                .collect()
            )),
        ])
    )
}
