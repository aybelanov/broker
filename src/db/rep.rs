use std::{collections::HashMap, error::Error};
use rusqlite::{params, OptionalExtension, Error as DbError };
use crate::models::{Source, Record};
use super::DB_POOL;

/// Gets a setting value by a key
pub fn get_setting_by_key(key:&str) -> Result<Option<String>, Box<dyn Error>>{
    let conn = DB_POOL.get()?;
    let mut query = conn.prepare("SELECT value FROM settings WHERE key = ?1")?; 
    let val = query.query_row(params![key], |row| Ok(row.get(0)?)).optional()?;
    Ok(val)
}

/// Gets all setting dictionary
pub fn get_all_setting() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let conn = DB_POOL.get()?;
    let mut query = conn.prepare("SELECT key, value FROM settings")?; 
    let rows = query.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    let settings: HashMap<String, String> = rows
        .collect::<Result<Vec<(String, String)>, DbError>>()?
        .into_iter()
        .collect();

    Ok(settings)
}

// data source repository methods
pub fn get_sources()->Result<Vec<Source>, Box<dyn Error>> {
    unimplemented!();
}

pub fn get_source_by_name(name: &str)-> Result<Source, Box<dyn Error>>{
    unimplemented!();
}

pub fn add_source(source: Source)->Result<Source, Box<dyn Error>>{
    unimplemented!();
}

pub fn update_source(source: Source)->Result<Source, Box<dyn Error>>{
    unimplemented!();
}

pub fn delete_source(name: &str)-> Result<(), Box<dyn Error>>{
    unimplemented!();
}

// sensor (source) data repository methods
pub fn get_data(count: u32)->Result<Vec<Record>, Box<dyn Error>>{
    unimplemented!();
}

pub fn add_data(records: Vec<Record>) -> Result<u32, Box<dyn Error>> {
    unimplemented!();
}

pub fn delete_data(ids: Vec<u32>){
    unimplemented!();
}