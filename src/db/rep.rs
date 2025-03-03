
use std::collections::HashMap;

use anyhow::Result;
use crate::models::{Source, Record};

// settings repository methods
pub fn get_setting_by_key(key:&str)->Result<String>{
    unimplemented!();
}

pub fn get_all_setting()->Result<HashMap<String, String>> {
    unimplemented!();
}

// data source repository methods
pub fn get_sources()->Result<Vec<Source>> {
    unimplemented!();
}

pub fn get_source_by_name(name: &str)-> Result<Source>{
    unimplemented!();
}

pub fn add_source(source: Source)->Result<Source>{
    unimplemented!();
}

pub fn update_source(source: Source)->Result<Source>{
    unimplemented!();
}

pub fn delete_source(name: &str){
    unimplemented!();
}

// sensor (source) data repository methods
pub fn get_data(count: u32)->Result<Vec<Record>>{
    unimplemented!();
}

pub fn add_data(records: Vec<Record>){
    unimplemented!();
}

pub fn delete_data(ids: Vec<u32>){
    unimplemented!();
}