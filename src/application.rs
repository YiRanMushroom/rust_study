use yiran_json::{parse_json, JsonNode};

#[derive(Debug, Clone, Default, yiran_json::JsonType, PartialEq)]
struct TestStruct {
    field1: i32,
    field2: f64,
    field3: String,
}

#[derive(Debug, Clone, Default, yiran_json::JsonType, PartialEq)]
struct TestStruct2 {
    field1: i32,
    field2: TestStruct,
}

#[derive(Debug, Clone, Default, yiran_json::JsonType, PartialEq)]
struct TpType(i32, i64, f64);

#[derive(Debug, Clone, Default, yiran_json::JsonType, PartialEq)]
enum TestEnum {
    #[default]
    Variant0,
    Variant1(),
    Variant2(String),
    Variant3(i32, f64),
    Variant4 {
        field1: i32,
        field2: f64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use yiran_json::*;

    #[test]
    fn test_tuple() {
        let tp = TpType(42, 3, 3.14);
        let tp_json = tp.to_json();
        let another_tp = TpType::from_json(&tp_json);
        assert_eq!(tp, another_tp);
    }

    #[test]
    fn test_enum() {
        use yiran_json::*;
        let first_variant = TestEnum::Variant1();
        let first_variant_json = first_variant.to_json();
        let another_first_variant = TestEnum::from_json(&first_variant_json);
        assert_eq!(first_variant, another_first_variant);

        let second_variant = TestEnum::Variant2("Hi, Dad!".to_string());
        let second_variant_json = second_variant.to_json();
        let another_second_variant = TestEnum::from_json(&second_variant_json);
        assert_eq!(second_variant, another_second_variant);

        let third_variant = TestEnum::Variant3(42, 3.14);
        let third_variant_json = third_variant.to_json();
        let another_third_variant = TestEnum::from_json(&third_variant_json);
        assert_eq!(third_variant, another_third_variant);

        let fourth_variant = TestEnum::Variant4 {
            field1: 42,
            field2: 3.14,
        };
        let fourth_variant_json = fourth_variant.to_json();
        let another_fourth_variant = TestEnum::from_json(&fourth_variant_json);
        assert_eq!(fourth_variant, another_fourth_variant);
    }

    #[test]
    fn test_struct() {
        let test_struct = TestStruct {
            field1: 42,
            field2: 3.14,
            field3: "Hi, Mom!".to_string(),
        };

        let test_struct_json = test_struct.to_json();
        let another_test_struct = TestStruct::from_json(&test_struct_json);
        assert_eq!(test_struct, another_test_struct);

        let test_struct2 = TestStruct2 {
            field1: 42,
            field2: another_test_struct.clone(),
        };

        let test_struct2_json = test_struct2.to_json();
        let another_test_struct2 = TestStruct2::from_json(&test_struct2_json);
        assert_eq!(test_struct2, another_test_struct2);
    }
}

#[derive(Debug, Clone, Default, yiran_json::JsonType, PartialEq)]
struct Friend {
    name: String,
    age: i32,
}

pub fn main() {
    use yiran_json::*;

    let first_name = "Yiran";
    let a = Friend {
        name: "A".to_string(),
        age: 20,
    };

    let b = Friend {
        name: "B".to_string(),
        age: 20,
    };

    let first_name_key = "first name";

    let mut input_str: String = String::default();

    std::io::stdin().read_line(&mut input_str).unwrap();

    let json = json_object! {
        #(first_name_key.to_string()): #(first_name.to_json()),
        "last name": "Wang",
        "age": 20,
        "is_student": true,
        "courses": [
            {
                "name": "CPEN 212",
                "credits": 4
            },
            {
                "name": "CPSC 221",
                "credits": 4
            },
            {
                "name": "Math 256",
                "credits": 3
            },
            {
                "name": "ELEC 201",
                "credits": 4
            },
            {
                "name": "BIOL 112",
                "credits": 3
            }
        ],
        "address": {
            "street": "1935 Lower Mall",
            "city": "Vancouver",
            "postal_code": "V6T 1X1"
        },
        "friends": [
            #(a.to_json()),
            #(b.to_json())
        ],
        "graduated": false,
        "university": "UBC",
        "message": ["Hi, mom!", "Hello, World!", "Rustrover is the goat"],
        "additional_message" : #(input_str.trim().to_json())
    };
    println!("{}", json.dump(2, false));
}
