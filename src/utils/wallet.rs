use bitcoin::{key::{Secp256k1, UncompressedPublicKeyError}, secp256k1::{All, SecretKey}, Address, CompressedPublicKey, KnownHrp, Network, PrivateKey};
use rand::{rngs::ThreadRng};

pub struct Wallet {
    prv: PrivateKey,
    p2pkh_address: String,
    p2wpkh_address: String
}

impl Wallet {
    pub fn new_random(secp: Secp256k1<All>, rng: &mut ThreadRng) -> Result<Wallet, UncompressedPublicKeyError> {
        let private_key = PrivateKey::new(SecretKey::new(rng), Network::Bitcoin);

        match CompressedPublicKey::from_private_key(&secp, &private_key) {
            Ok(compressed_key) => {
                let p2pkh = Address::p2pkh(compressed_key, Network::Bitcoin);
                let p2wpkh = Address::p2wpkh(&compressed_key, KnownHrp::Mainnet);
                return Ok(Wallet { prv: private_key, p2pkh_address: p2pkh.to_string(), p2wpkh_address: p2wpkh.to_string() });
            },

            Err(e) => return Err(e)
        }
    }
}

impl ToString for Wallet {
    fn to_string(&self) -> String {
        format!("Private Key: {}\nP2PKH Address: {}\nP2WPKH Address: {}\n", self.prv.to_wif(), self.p2pkh_address, self.p2wpkh_address)
    }
}