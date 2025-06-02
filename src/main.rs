use std::time;

mod utils;

use utils::file_handling::{create_address_set, create_address_set_par, read_from_csv};

fn main() {
    let mut time = time::Instant::now();
    let result = read_from_csv("blockchair_bitcoin_addresses_latest.tsv", b'\t');
    println!("Time elapsed: {:?}", time.elapsed());
    time = time::Instant::now();
    let dataset = create_address_set_par(result);
    println!("Time elapsed: {:?}", time.elapsed());
    println!("{} Items in Dataset", dataset.len().clone());
    println!("First a few addresses in dataset:");
    for i in dataset.iter().take(10) {
        println!("{}", i);
    }
}


#[cfg(test)]
mod tests;