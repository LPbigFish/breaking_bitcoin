use bitcoin::{hex::{Case, DisplayHex}, opcodes::{all::*, *}, Address, KnownHrp, Network, Script};

#[derive(Clone, Debug)]
pub struct ScriptBuild {
    iteration: u128,
    p2sh: Address,
    p2wsh: Address
}

impl ScriptBuild {
    pub fn new(i: u128) -> ScriptBuild {
        let script_builder = Script::builder()
        .push_slice(i.to_le_bytes())
        .push_opcode(OP_DROP)
        .push_opcode(OP_TRUE)
        .push_verify();

        let script = script_builder.as_script();

        println!("{}", script.to_hex_string());

        let p2wsh = Address::p2wsh(script, KnownHrp::Mainnet);
        let p2sh = Address::p2sh(script, Network::Bitcoin).expect("Error while generating P2SH address");

        ScriptBuild { iteration: i, p2sh, p2wsh }
    }
}

impl ToString for ScriptBuild {
    fn to_string(&self) -> String {
        format!("Script: OP_PUSHBYTES_16 00000000000000000000000000000000 (iteration.to_le_bytes()) OP_DROP OP_PUSHNUM_1 OP_VERIFY\nMagic number: {}\nP2SH Address: {}\nP2WSH Address: {}\n", self.iteration, self.p2sh, self.p2wsh)
    }
}