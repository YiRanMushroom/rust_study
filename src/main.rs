use crate::app::json_basic::{FromAndToJson, Json, JsonNode};
use std::io::stdin;

mod app;
use crate::app::{json_lexer, json_parser, parse_json};

extern crate macros;
use macros::{print_struct_info, JsonStruct};

#[print_struct_info]
#[JsonStruct]
#[derive(Debug, Clone)]
struct TestStruct {
    field1: i32,
    field2: f64,
    field3: String,
}

#[JsonStruct]
#[derive(Debug)]
struct TestStruct2 {
    field1: i32,
    field2: TestStruct,
}

fn main() {
    println!("Hello, world!");

    // let hash_map = std::collections::HashMap::new();

    let mut input = String::new();

    let json_str = "{\"name\":\"Alice\",\"age\":30,\"is_student\":false,\"courses\":[{\"name\":\"Math\",\"credits\":3},{\"name\":\"Science\",\"credits\":4},{\"name\":\"History\",\"credits\":2}],\"address\":{\"street\":\"123 Main St\",\"city\":\"Wonderland\",\"postal_code\":\"12345\"},\"friends\":[{\"name\":\"Bob\",\"age\":28},{\"name\":\"Charlie\",\"age\":35}],\"graduated\":null}";

    let test_struct = TestStruct {
        field1: 42,
        field2: 3.14,
        field3: "Hello, world!".to_string(),
    };

    let test_struct_json = Json::new(test_struct.to_json());

    let another_test_struct = TestStruct::from_json(test_struct_json.get_root());

    let test_struct2 = TestStruct2 {
        field1: 42,
        field2: another_test_struct.clone(),
    };

    println!("{}", test_struct_json);
    println!("{:?}", another_test_struct);

    let test_struct2_json = Json::new(test_struct2.to_json());
    println!("{}", test_struct2_json);

    let another_test_struct2 = TestStruct2::from_json(test_struct2_json.get_root());
    println!("{:?}", another_test_struct2);

    let mut json = parse_json(json_str).unwrap();

    json["name".to_string()] = JsonNode::String("Bob".to_string());
    json["courses".to_string()][0]["credits".to_string()] = JsonNode::Null;

    println!("{}", json);
}
