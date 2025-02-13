use json;
use json::FromAndToJson;

#[derive(Debug, Clone, Default, json::JsonStruct)]
struct TestStruct {
    field1: i32,
    field2: f64,
    field3: String,
}

#[derive(Debug, Clone, Default, json::JsonStruct)]
struct TestStruct2 {
    field1: i32,
    field2: TestStruct,
}

pub fn main() {
    println!("Hello, world!");

    let json_str = "{\"name\":\"Alice\",\"age\":30.,\"is_student\":1e-2,\"courses\":[{\"name\":\"Math\",\"credits\":3e3},{\"name\":\"Science\",\"credits\":4},{\"name\":\"History\",\"credits\":2}],\"address\":{\"street\":\"123 Main St\",\"city\":\"Wonderland\",\"postal_code\":\"12345\"},\"friends\":[{\"name\":\"Bob\",\"age\":28},{\"name\":\"Charlie\",\"age\":35}],\"graduated\":null, \"message\":\"你好中国\"}";

    let test_struct = TestStruct {
        field1: 42,
        field2: 3.14,
        field3: "Hello, world!".to_string(),
    };

    let test_struct_json = test_struct.to_json().move_as_root();

    let another_test_struct = TestStruct::from_json(test_struct_json.get_root());

    let test_struct2 = TestStruct2 {
        field1: 42,
        field2: another_test_struct.clone(),
    };

    println!("{}", test_struct_json);
    println!("{:?}", another_test_struct);

    let test_struct2_json = test_struct2.to_json().move_as_root();
    println!("{}", test_struct2_json);

    let another_test_struct2 = TestStruct2::from_json(test_struct2_json.get_root());
    println!("{:?}", another_test_struct2);

    let mut json = json::parse_json(json_str).unwrap();

    json["name".to_string()] = json::JsonNode::String("Bob".to_string());
    json["courses".to_string()][0]["credits".to_string()] = json::JsonNode::Null;

    for (idx, v) in json.obj_iter().unwrap().enumerate() {
        println!("[{}]: <\"{}\": {}>", idx, v.0, v.1);
    }
}
