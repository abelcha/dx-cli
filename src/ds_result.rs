use color_print::cprintln;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use strum_macros::EnumString;

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
}

pub fn save_result_data(dsfile: &str, result_dat: ResultData) {
    let mut manager = RESULT_DATA_MANAGER.lock().unwrap();
    manager.dsfiles.insert(dsfile.to_string(), result_dat);
}

#[derive(Debug, Clone, EnumString)]
pub enum CacheStatus {
    CacheHit(i64),
    CacheMiss,
    NotFound,
}

pub fn get_cached_property(dsfile: &str, path: &str, key: &str) -> Result<CacheStatus, String> {
    let manager = RESULT_DATA_MANAGER.lock().unwrap();
    match manager.dsfiles.get(dsfile) {
        Some(value) => match value.get_file_property(path, key) {
            Ok(val) => Ok(CacheStatus::CacheHit(val)),
            Err(err) => Ok(CacheStatus::CacheMiss),
        },
        None => Ok(CacheStatus::NotFound),
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(u64),
    Float(f64),
    Text(String),
    Boolean(bool),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResultData {
    files: RecordMap,
}

impl ResultData {
    pub fn new() -> Self {
        ResultData {
            files: HashMap::new(),
        }
    }

    pub fn get_file_property(&self, file: &str, key: &str) -> Result<i64, String> {
        if (!self.files.contains_key(file)) {
            return Err("folder not found".to_string());
        }

        match self.files.get(file).unwrap().get(key) {
            Some(value) => Ok(*value as i64),
            None => Err("Key not found".to_string()),
        }
    }

    pub fn get_files(&self) -> &HashMap<String, HashMap<String, u64>> {
        &self.files
    }

    pub fn add_record(&mut self, file: &str, key: &str, value: u64) {
        let file_properties = self
            .files
            .entry(file.to_string())
            .or_insert_with(HashMap::new);
        file_properties.insert(key.to_string(), value);
    }
}
