use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs::File, io::{BufReader, BufWriter, Write}, time};
use rayon::prelude::*;

use super::wallet::Wallet;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Record {
    pub(crate) address: String,
    pub(crate) balance: u64
}

pub fn read_from_csv(path: &str, delim: u8) -> Vec<Record> {
    let mut result: Vec<Record> = Vec::new();
    
    let file = if let Ok(file) = File::open(path) {
        file
    } else {
        panic!("Can't open file on path: {}", path);
    };

    let buf_read = BufReader::new(file);
    
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delim)
        .from_reader(buf_read);

    for res in reader.deserialize::<Record>() {
        if let Ok(rec) = res {
            result.push(rec);
        }
    }
    
    result
}

pub fn create_address_set(record_vec: &Vec<Record>) -> HashSet<String> {
    let mut result: HashSet<String> = HashSet::with_capacity(record_vec.len());

    for rec in record_vec {
        result.insert(rec.address.clone());
    }

    result
}

pub fn create_address_set_par(record_vec: Vec<Record>) -> HashSet<String> {
    let cap = record_vec.len();
    let mut result = HashSet::with_capacity(cap);

    result.par_extend(record_vec
        .into_par_iter()
        .map(|rec| rec.address)
    );
    result
}

pub fn write_the_key(wallet: Wallet) {
    if let Ok(mut file) = File::create(format!("{:?}.{}.json", time::SystemTime::now(), wallet.p2pkh_address)) {
        let data = serde_json::to_string_pretty(&wallet).expect("Failed to parse wallet data to JSON!");
        file.write_all(data.as_bytes()).expect("Failed to write wallet data to a file!");
    }
}