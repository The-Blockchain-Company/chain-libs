use crate::{legacy::UtxoDeclaration, value::Value};
use bcc_legacy_address::Addr;
use bcc_legacy_address::ExtendedAddr;
use ed25519_bip32::{XPub, XPUB_SIZE};
use rand_core::RngCore;

#[derive(Default)]
pub struct OldAddressBuilder;

impl OldAddressBuilder {
    pub fn build_utxo_declaration(size: Option<usize>) -> UtxoDeclaration {
        let nb = match size {
            Some(size) => size,
            None => {
                let mut rng = rand_core::OsRng;
                let nb: usize = rng.next_u32() as usize;
                nb % 255
            }
        };
        let mut addrs = Vec::with_capacity(nb);
        for _ in 0..nb {
            addrs.push(Self::build_old_address());
        }
        UtxoDeclaration { addrs }
    }

    pub fn build_old_address() -> (Addr, Value) {
        // some reasonable value
        let mut rng = rand_core::OsRng;
        let value = Value(rng.next_u64() % 2_000_000 + 1);
        let xpub = {
            let mut buf = [0u8; XPUB_SIZE];
            rng.fill_bytes(&mut buf);
            XPub::from_bytes(buf)
        };
        let ea = ExtendedAddr::new_simple(&xpub, None);
        (ea.to_address(), value)
    }
}
