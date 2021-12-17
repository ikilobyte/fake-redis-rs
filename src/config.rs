use crate::parser::get;
use crate::parser::set;
use crate::Protocol;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PARSE_LIST: HashMap<&'static str, fn(Vec<String>) -> Protocol> = {
        let mut map: HashMap<&str, fn(Vec<String>) -> Protocol> = HashMap::new();
        map.insert("SET", set::transform);
        map.insert("GET", get::transform);
        map
    };
}
