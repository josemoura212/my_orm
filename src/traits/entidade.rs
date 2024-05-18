use serde_json::Value;
use std::collections::HashMap;

#[allow(unused)]
pub trait TEntidade {
    fn id(&self) -> i32;
    fn campos_model() -> Vec<(String, String)>;
    fn from_data_row(data: HashMap<String, Value>) -> Result<Box<Self>, String>;
}
