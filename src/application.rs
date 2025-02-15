use json;
use json::FromAndToJson;

#[derive(Debug, Clone, Default, json::JsonType, PartialEq)]
struct TestStruct {
    field1: i32,
    field2: f64,
    field3: String,
}

#[derive(Debug, Clone, Default, json::JsonType, PartialEq)]
struct TestStruct2 {
    field1: i32,
    field2: TestStruct,
}

#[derive(Debug, Clone, Default, json::JsonType, PartialEq)]
struct TpType(i32, i64, f64);

#[derive(Debug, Clone, Default, json::JsonType, PartialEq)]
enum TestEnum {
    #[default]
    Variant1,
    Variant2(String),
    Variant3(i32, f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple() {
        let tp = TpType(42, 3, 3.14);
        let tp_json = tp.to_json().move_as_root();
        let another_tp = TpType::from_json(tp_json.get_root());
        assert_eq!(tp, another_tp);
    }

    #[test]
    fn test_enum() {
        let first_variant = TestEnum::Variant1;
        let first_variant_json = first_variant.to_json().move_as_root();
        let another_first_variant = TestEnum::from_json(first_variant_json.get_root());
        assert_eq!(first_variant, another_first_variant);

        let second_variant = TestEnum::Variant2("Hi, Dad!".to_string());
        let second_variant_json = second_variant.to_json().move_as_root();
        let another_second_variant = TestEnum::from_json(second_variant_json.get_root());
        assert_eq!(second_variant, another_second_variant);

        let third_variant = TestEnum::Variant3(42, 3.14);
        let third_variant_json = third_variant.to_json().move_as_root();
        let another_third_variant = TestEnum::from_json(third_variant_json.get_root());
        assert_eq!(third_variant, another_third_variant);
    }

    #[test]
    fn test_struct() {
        let test_struct = TestStruct {
            field1: 42,
            field2: 3.14,
            field3: "Hi, Mom!".to_string(),
        };

        let test_struct_json = test_struct.to_json().move_as_root();
        let another_test_struct = TestStruct::from_json(test_struct_json.get_root());
        assert_eq!(test_struct, another_test_struct);

        let test_struct2 = TestStruct2 {
            field1: 42,
            field2: another_test_struct.clone(),
        };

        let test_struct2_json = test_struct2.to_json().move_as_root();
        let another_test_struct2 = TestStruct2::from_json(test_struct2_json.get_root());
        assert_eq!(test_struct2, another_test_struct2);
    }
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
