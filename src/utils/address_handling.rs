use bitcoin::AddressType;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::csv::Record;

#[derive(Default)]
pub(crate) struct AddressBucket {
    pub(crate) p2pkh: Vec<Record>,
    pub(crate) p2sh: Vec<Record>,
    pub(crate) p2wpkh: Vec<Record>,
    pub(crate) p2wsh: Vec<Record>,
    pub(crate) p2tr: Vec<Record>,
    pub(crate) other: Vec<Record>
}

impl AddressBucket {
    fn absorb(&mut self, mut next: AddressBucket) {
        self.p2pkh.append(&mut next.p2pkh);
        self.p2sh .append(&mut next.p2sh );
        self.p2wpkh.append(&mut next.p2wpkh);
        self.p2wsh.append(&mut next.p2wsh);
        self.p2tr .append(&mut next.p2tr );
        self.other.append(&mut next.other)
    }
}

#[inline]
fn classify(addr: &str) -> Option<AddressType> {
    let bytes = addr.as_bytes();
    match bytes[0] {
        b'1' => Some(AddressType::P2pkh),
        b'3' => Some(AddressType::P2sh),
        b'b' => {
            match bytes[3] {
                b'p' | b'P' => Some(AddressType::P2tr),
                _ => match addr.len() {
                    42 => Some(AddressType::P2wpkh),
                    _  => Some(AddressType::P2wsh),
                },
            }
        }
        _ => None
    }
}

pub fn filter_addresses(records: Vec<Record>) -> AddressBucket {
    records.par_iter().fold(AddressBucket::default, |mut acc, rec| {
        if let Some(add_type) = classify(&rec.address) {
            match add_type {
                AddressType::P2pkh  => acc.p2pkh.push(rec.clone()),
                AddressType::P2sh => acc.p2sh.push(rec.clone()),
                AddressType::P2tr => acc.p2tr.push(rec.clone()),
                AddressType::P2wpkh => acc.p2wpkh.push(rec.clone()),
                AddressType::P2wsh => acc.p2wsh.push(rec.clone()),
                _ => ()
            }
        } else {
            acc.other.push(rec.clone());
        }
        acc
    }).reduce(AddressBucket::default, |mut left, right| {
        left.absorb(right);
        left
    })
}