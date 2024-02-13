use color_print::cprintln;
use lazy_static::lazy_static;
use strum_macros::EnumString;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::ds_parser::ModType;

type RecordMap = HashMap<String, HashMap<String, u64>>;
type RecordMapHistory = HashMap<String, ResultData>;

#[derive(Debug, Default, Clone)]
pub struct ResultDataManager {
    dsfiles: RecordMapHistory,
}

lazy_static! {
    pub static ref RESULT_DATA_MANAGER: Mutex<ResultDataManager> =
        Mutex::new(ResultDataManager::default());
}

impl ResultDataManager {
    pub fn new() -> Self {
        ResultDataManager {
            dsfiles: HashMap::new(),
        }
    }

    // pub fn get_cache_or_create_new(&mut self, path: &str) -> ResultData {
    //     if !self.dsfiles.contains_key(path) {
    //         println!("[ResultDataManager] - Create NEW {}", path);
    //         let new_result = ResultData::new();
    //         self.dsfiles.insert(path.to_string(), new_result);
    //         return new_result;
    //     }
    //     println!("[ResultDataManager] - get cache {}", path);
    //     // At this point, the key is definitely present, so `unwrap` is safe
    //     return &self.dsfiles.get(path).unwrap().clone();
    // }
}

pub fn save_result_data(dsfile: &str, result_dat: ResultData) {
    let mut manager = RESULT_DATA_MANAGER.lock().unwrap();
    // println!("Insert {}", dsfile.to_string());
    manager.dsfiles.insert(dsfile.to_string(), result_dat);
    // println!("TOTAL LENNNN -----> {} ", manager.dsfiles.len());
}

#[derive(Debug, Clone, EnumString)]
pub enum CacheStatus {
    CacheHit(i64),
    CacheMiss,
    NotFound,
}

pub fn get_cached_property(dsfile: &str, path: &str, key: &str) -> Result<CacheStatus, String> {
    let manager = RESULT_DATA_MANAGER.lock().unwrap();
    // let already_parsed = manager.dsfiles.get(dsfile);
    match manager.dsfiles.get(dsfile)  {
        Some(value) => {
            // println!("ALREADY PARSED {}", path);
            let zz = value.get_file_property(path, key);
            match zz {
                Ok(val) => Ok(CacheStatus::CacheHit(val)),
                Err(err) => Ok(CacheStatus::CacheMiss),
            }
        },
        None => {
            // println!("No cache found {}", path);
            return Ok(CacheStatus::NotFound);
            // return Err("No cache found".to_string());
        }
    }
    // if (already_parsed.is_none()) {
    //     return Err("No cache found".to_string());
    // }
    // return already_parsed.get_file_property(path, key);
}

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
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResultData {
    files: RecordMap,
}

impl ResultData {
    // Constructor to initialize the ResultData
    pub fn new() -> Self {
        ResultData {
            files: HashMap::new(),
        }
    }

    pub fn get_file_property(&self, file: &str, key: &str) -> Result<i64, String> {
        // println!("[get_file_property] - file: {}", file);
        if (!self.files.contains_key(file)) {
            // println!("[get_file_property] - not contains: {}", file);
            return Err("folder not found".to_string());
        }
        // println!("[get_file_property] - YESYES contain: {}", file);

        // return 0;
        return match self.files.get(file).unwrap().get(key) {
            Some(value) => Ok(*value as i64),
            None => Err("Key not found".to_string()),
        };
        // let filx =
        // match self.files.get(file) {
        //     Some(file_properties) => file_properties.get(key),
        //     None => None,
        // }
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
