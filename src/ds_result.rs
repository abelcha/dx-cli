use std::collections::HashMap;

// Define an enum for the possible value types
#[derive(Debug, Clone)]
pub enum Value {
    Integer(u64),
    Float(f64),
    Text(String),
    Boolean(bool),
    // Add more types as needed
}

// Define the ResultData class to handle multiple files
#[derive(Debug, Clone)]
pub struct ResultData {
    files: HashMap<String, HashMap<String, u64>>,
}

impl ResultData {
    // Constructor to initialize the ResultData
    pub fn new() -> Self {
        ResultData {
            files: HashMap::new(),
        }
    }

    pub fn get_files(&self) -> &HashMap<String, HashMap<String, u64>> {
        &self.files
    }

    // Method to add a property to a specific file
    pub fn add_record(&mut self, file: &str, key: &str, value: u64) {
        let file_properties = self
            .files
            .entry(file.to_string())
            .or_insert_with(HashMap::new);
        file_properties.insert(key.to_string(), value);
    }
}
