use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::thread::panicking;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum JsonNode {
    Object(HashMap<String, JsonNode>),
    Array(Vec<JsonNode>),
    String(String),
    Number(f64),
    Boolean(bool),
    #[default]
    Null,
}

impl JsonNode {
    pub fn new() -> JsonNode {
        JsonNode::Null
    }
}

impl Display for JsonNode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            JsonNode::Object(obj) => {
                write!(f, "{{")?;
                for (idx, (k, v)) in obj.iter().enumerate() {
                    write!(f, "\"{}\": ", k)?;
                    v.fmt(f)?;
                    if idx != obj.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")?;
            }
            JsonNode::Array(arr) => {
                write!(f, "[")?;
                for (idx, v) in arr.iter().enumerate() {
                    v.fmt(f)?;
                    if idx != arr.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
            }
            JsonNode::String(s) => {
                write!(f, "\"{}\"", s)?;
            }
            JsonNode::Number(n) => {
                write!(f, "{}", n)?;
            }
            JsonNode::Boolean(b) => {
                write!(f, "{}", b)?;
            }
            JsonNode::Null => {
                write!(f, "null")?;
            }
        };

        Ok(())
    }
}

impl<'a> Index<&'a str> for JsonNode {
    type Output = JsonNode;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self {
            JsonNode::Object(obj) => {
                if obj.contains_key(index) {
                    obj.get(index).unwrap()
                } else {
                    panic!("Key not found")
                }
            }
            _ => panic!("Cannot index non-object type"),
        }
    }
}

impl<'a> IndexMut<&'a str> for JsonNode {
    fn index_mut(&mut self, index: &'a str) -> &mut Self {
        match self {
            JsonNode::Object(obj) => {
                if obj.contains_key(index) {
                    obj.get_mut(index).unwrap()
                } else {
                    obj.insert(index.to_string(), JsonNode::Null);
                    obj.get_mut(index).unwrap()
                }
            }
            _ => panic!("Cannot index non-object type"),
        }
    }
}

impl Index<usize> for JsonNode {
    type Output = JsonNode;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            JsonNode::Array(arr) => {
                if index < arr.len() {
                    arr.get(index).unwrap()
                } else {
                    panic!("Index out of bounds")
                }
            }
            _ => panic!("Cannot index non-array type"),
        }
    }
}

impl IndexMut<usize> for JsonNode {
    fn index_mut(&mut self, index: usize) -> &mut Self {
        match self {
            JsonNode::Array(arr) => {
                if index < arr.len() {
                    arr.get_mut(index).unwrap()
                } else {
                    panic!("Index out of bounds")
                }
            }
            _ => panic!("Cannot index non-array type"),
        }
    }
}

pub trait FromJson {
    fn from_json(json: &JsonNode) -> Self;
}

pub trait ToJson {
    fn to_json(&self) -> JsonNode;
}

impl FromJson for String {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::String(s) => s.clone(),
            _ => panic!("Cannot convert non-string type to string"),
        }
    }
}

impl ToJson for String {
    fn to_json(&self) -> JsonNode {
        JsonNode::String(self.clone())
    }
}

/*impl FromAndToJson for Vec<JsonNode> {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Array(arr) => arr.clone(),
            _ => panic!("Cannot convert non-array type to array"),
        }
    }

    fn to_json(&self) -> JsonNode {
        JsonNode::Array(self.clone())
    }
}*/
impl<T> FromJson for Vec<T>
where
    T: FromJson,
{
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Array(arr) => arr.iter().map(|x| T::from_json(x)).collect(),
            _ => panic!("Cannot convert non-array type to array"),
        }
    }
}

impl<T> ToJson for Vec<T>
where
    T: ToJson,
{
    fn to_json(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|x| x.to_json()).collect())
    }
}

/*impl FromAndToJson for HashMap<String, JsonNode> {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Object(obj) => obj.clone(),
            _ => panic!("Cannot convert non-object type to object"),
        }
    }

    fn to_json(&self) -> JsonNode {
        JsonNode::Object(self.clone())
    }
}*/

impl<T> FromJson for HashMap<String, T>
where
    T: FromJson,
{
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Object(obj) => obj
                .iter()
                .map(|(k, v)| (k.clone(), T::from_json(&v)))
                .collect(),
            _ => panic!("Cannot convert non-object type to object"),
        }
    }
}

impl<T> ToJson for HashMap<String, T>
where
    T: ToJson,
{
    fn to_json(&self) -> JsonNode {
        JsonNode::Object(self.iter().map(|(k, v)| (k.clone(), v.to_json())).collect())
    }
}

impl<T> ToJson for HashMap<&str, T>
where
    T: ToJson,
{
    fn to_json(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(k, v)| (k.to_string(), v.to_json()))
                .collect(),
        )
    }
}

/*impl FromAndToJson for bool {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Boolean(b) => b.clone(),
            _ => panic!("Cannot convert non-boolean type to boolean"),
        }
    }

    fn to_json(&self) -> JsonNode {
        JsonNode::Boolean(self.clone())
    }
}*/

impl FromJson for bool {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Boolean(b) => b.clone(),
            _ => panic!("Cannot convert non-boolean type to boolean"),
        }
    }
}

impl ToJson for bool {
    fn to_json(&self) -> JsonNode {
        JsonNode::Boolean(self.clone())
    }
}

// all number types
/*impl FromAndToJson for f64 {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Number(n) => n.clone(),
            _ => panic!("Cannot convert non-number type to number"),
        }
    }

    fn to_json(&self) -> JsonNode {
        JsonNode::Number(self.clone())
    }
}*/

impl FromJson for f64 {
    fn from_json(json: &JsonNode) -> Self {
        match json {
            JsonNode::Number(n) => n.clone(),
            _ => panic!("Cannot convert non-number type to number"),
        }
    }
}

impl ToJson for f64 {
    fn to_json(&self) -> JsonNode {
        JsonNode::Number(self.clone())
    }
}

pub struct JsonObjIterRef<'a> {
    map_iter: std::collections::hash_map::Iter<'a, String, JsonNode>,
}

pub struct JsonObjIterMut<'a> {
    map_iter: std::collections::hash_map::IterMut<'a, String, JsonNode>,
}

pub struct JsonObjIntoIter {
    map_iter: std::collections::hash_map::IntoIter<String, JsonNode>,
}

pub struct JsonArrIntoIter {
    vec_iter: std::vec::IntoIter<JsonNode>,
}

impl<'a> Iterator for JsonObjIterRef<'a> {
    type Item = (&'a String, &'a JsonNode);

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next()
    }
}

impl<'a> Iterator for JsonObjIterMut<'a> {
    type Item = (&'a String, &'a mut JsonNode);

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next()
    }
}

impl Iterator for JsonObjIntoIter {
    type Item = (String, JsonNode);

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next()
    }
}
impl Iterator for JsonArrIntoIter {
    type Item = JsonNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec_iter.next()
    }
}

impl JsonNode {
    pub fn obj_iter(&self) -> Option<JsonObjIterRef> {
        match self {
            JsonNode::Object(obj) => Some(JsonObjIterRef {
                map_iter: obj.iter(),
            }),
            _ => None,
        }
    }

    pub fn obj_iter_mut(&mut self) -> Option<JsonObjIterMut> {
        match self {
            JsonNode::Object(obj) => Some(JsonObjIterMut {
                map_iter: obj.iter_mut(),
            }),
            _ => None,
        }
    }

    pub fn obj_into_iter(self) -> Option<JsonObjIntoIter> {
        match self {
            JsonNode::Object(obj) => Some(JsonObjIntoIter {
                map_iter: obj.into_iter(),
            }),
            _ => None,
        }
    }

    pub fn arr_iter(&self) -> Option<std::slice::Iter<JsonNode>> {
        match self {
            JsonNode::Array(arr) => Some(arr.iter()),
            _ => None,
        }
    }

    pub fn arr_iter_mut(&mut self) -> Option<std::slice::IterMut<JsonNode>> {
        match self {
            JsonNode::Array(arr) => Some(arr.iter_mut()),
            _ => None,
        }
    }

    pub fn arr_into_iter(self) -> Option<JsonArrIntoIter> {
        match self {
            JsonNode::Array(arr) => Some(JsonArrIntoIter {
                vec_iter: arr.into_iter(),
            }),
            _ => None,
        }
    }

    pub fn insert(&mut self, key: String, value: JsonNode) {
        match self {
            JsonNode::Object(obj) => {
                obj.insert(key, value);
            }
            _ => panic!("Cannot insert into non-object type"),
        }
    }

    pub fn push(&mut self, value: JsonNode) {
        match self {
            JsonNode::Array(arr) => {
                arr.push(value);
            }
            _ => panic!("Cannot push into non-array type"),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        match self {
            JsonNode::Array(arr) => {
                arr.reserve(additional);
            }
            _ => panic!("Cannot reserve into non-array type"),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            JsonNode::Object(obj) => obj.len(),
            JsonNode::Array(arr) => arr.len(),
            _ => panic!("Cannot get length of non-object or non-array type"),
        }
    }

    pub fn resize(&mut self, new_len: usize) {
        match self {
            JsonNode::Array(arr) => {
                arr.resize(new_len, JsonNode::Null);
            }
            _ => panic!("Cannot resize non-array type"),
        }
    }

    pub fn clear(&mut self) {
        match self {
            JsonNode::Object(obj) => {
                obj.clear();
            }
            JsonNode::Array(arr) => {
                arr.clear();
            }
            _ => panic!("Cannot clear non-object or non-array type"),
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<JsonNode> {
        match self {
            JsonNode::Object(obj) => obj.remove(key),
            _ => panic!("Cannot remove from non-object type"),
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            JsonNode::Object(obj) => obj.contains_key(key),
            _ => panic!("Cannot check key in non-object type"),
        }
    }

    pub fn set_null(&mut self) {
        *self = JsonNode::Null;
    }
}

macro_rules! impl_from_and_to_json_for_number {
    ($($t:ty),*) => {
        $(
            impl FromJson for $t {
                fn from_json(json: &JsonNode) -> Self {
                    match json {
                        JsonNode::Number(n) => n.clone() as $t,
                        _ => panic!("Cannot convert non-number type to number"),
                    }
                }
            }

            impl ToJson for $t {
                fn to_json(&self) -> JsonNode {
                    JsonNode::Number(self.clone() as f64)
                }
            }
        )*
    };
}

/*
    (pattern) => {

    };
*/

impl_from_and_to_json_for_number!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32
);

impl ToJson for &str {
    fn to_json(&self) -> JsonNode {
        JsonNode::String(self.to_string())
    }
}

impl FromJson for JsonNode {
    fn from_json(json: &JsonNode) -> Self {
        json.clone()
    }
}

impl ToJson for JsonNode {
    fn to_json(&self) -> JsonNode {
        self.clone()
    }
}
