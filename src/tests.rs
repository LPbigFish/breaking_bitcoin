use std::{collections::HashSet, sync::{Arc, RwLock}};

use bitcoin::key::Secp256k1;

use crate::utils::{address_handling::filter_addresses, file_handling::{create_address_set, create_address_set_par, read_from_csv}, kernel::{check_address, gac_range}, wallet::Wallet};

#[test]
fn create_wallet() {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    match Wallet::new_random(secp, &mut rng) {
        Ok(wallet) => {
            println!("{}", wallet.to_string());
            assert!(true);
        }
        Err(e) => {
            println!("{}", e.to_string());
            assert!(false);
        }
    }
    assert!(true);
}

#[test]
fn read_from_csv_test() {
    let path = "blockchair_bitcoin_addresses_latest.tsv";
    let result = read_from_csv(path, b'\t');
    println!("{} records loaded!", result.len());
    assert!(true);
}

#[test]
fn create_dataset_test() {
    let path = "blockchair_bitcoin_addresses_latest.tsv";
    let mut result = read_from_csv(path, b'\t');
    println!("{} records loaded!", result.len());
    let dataset = create_address_set(&mut result);
    println!("{} records converted and prepared", dataset.len());
    assert!(true);
}

#[test]
fn create_dataset_par_test() {
    let path = "blockchair_bitcoin_addresses_latest.tsv";
    let result = read_from_csv(path, b'\t');
    println!("{} records loaded!", result.len());
    let dataset = create_address_set_par(result);
    println!("{} records converted and prepared", dataset.len());
    assert!(true);
}

#[test]
fn sort_out_addresses_test() {
    let path = "blockchair_bitcoin_addresses_latest.tsv";
    let result = read_from_csv(path, b'\t');
    println!("{} records loaded!", result.len());
    let sorted_addresses = filter_addresses(result);
    println!("P2PKH: {} with total balance of: {:.2} BTC", sorted_addresses.p2pkh.len(), sorted_addresses.p2pkh.iter().fold(0u64, |mut val, rec| { val += rec.balance; val}) as f64 / 100_000_000.0 );
    println!("P2WPKH: {} with total balance of: {:.2} BTC", sorted_addresses.p2wpkh.len(), sorted_addresses.p2wpkh.iter().fold(0u64, |mut val, rec| { val += rec.balance; val}) as f64 / 100_000_000.0);
    println!("P2SH: {} with total balance of: {:.2} BTC", sorted_addresses.p2sh.len(), sorted_addresses.p2sh.iter().fold(0u64, |mut val, rec| { val += rec.balance; val}) as f64 / 100_000_000.0);
    println!("P2WSH: {} with total balance of: {:.2} BTC", sorted_addresses.p2wsh.len(), sorted_addresses.p2wsh.iter().fold(0u64, |mut val, rec| { val += rec.balance; val}) as f64 / 100_000_000.0);
    println!("P2TR: {} with total balance of: {:.2} BTC", sorted_addresses.p2tr.len(), sorted_addresses.p2tr.iter().fold(0u64, |mut val, rec| { val += rec.balance; val}) as f64 / 100_000_000.0);
    println!("Other: {} with total balance of: {:.2} BTC", sorted_addresses.other.len(), sorted_addresses.other.iter().fold(0u64, |mut val, rec| { val += rec.balance; val}) as f64 / 100_000_000.0);
    assert!(true);
}

#[test]
fn kernel_run_test() {
    let path = "blockchair_bitcoin_addresses_latest.tsv";
    println!("Loading data...");
    let result = read_from_csv(path, b'\t');
    println!("{} records loaded!", result.len());
    let dataset = create_address_set_par(result);
    let par_dataset = Arc::new(RwLock::new(dataset));
    println!("Running the kernel...");
    gac_range(par_dataset, 0..10);
    assert!(true);
}

#[test]
fn kernel_check_test() {
    let mut dataset: HashSet<String> = HashSet::new();
    dataset.insert("1K6KoYC69NnafWJ7YgtrpwJxBLiijWqwa6".to_string());
    let par_dataset = Arc::new(RwLock::new(dataset));

    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let mut wallet = Wallet::new_random(secp, &mut rng).unwrap();

    wallet.p2pkh_address = "1K6KoYC69NnafWJ7YgtrpwJxBLiijWqwa6".to_string();

    assert_eq!(true, check_address(&par_dataset, &wallet));
}