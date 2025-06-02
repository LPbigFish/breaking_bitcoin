use std::{
    collections::HashSet,
    ops::RangeBounds,
    sync::{Arc, RwLock},
};

use bitcoin::key::Secp256k1;

use super::{file_handling::write_the_key, wallet::Wallet};

pub fn gac_range<T: Sized + Ord, U: RangeBounds<T> + IntoIterator>(dict: Arc<RwLock<HashSet<String>>>, range: U) {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();

    for _ in range {
        if let Ok(wallet) = Wallet::new_random(secp.clone(), &mut rng) {
            if check_address(&dict, &wallet) {
                write_the_key(wallet);
            }
        }
    }
}

#[inline]
pub fn check_address(dict: &Arc<RwLock<HashSet<String>>>, wallet: &Wallet) -> bool {
    if let Ok(dec) = dict.try_read() {
        let r1 = dec.contains(&wallet.p2pkh_address);
        let r2 = dec.contains(&wallet.p2wpkh_address);
        println!("{}\n", wallet.to_string());
        return r1 || r2;
    }
    false
}