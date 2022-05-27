use std::collections::HashMap;

#[derive(Debug)]
pub struct Data {
    pub filepath: String,
    pub data: HashMap<String, String>
}

impl Data {
    pub fn new(path: &String) -> Result<Data, String> {

    }
}