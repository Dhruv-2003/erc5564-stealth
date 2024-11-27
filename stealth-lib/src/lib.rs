// Goal is to export the methods for a wasm binary from the eth-stealth-addresses lib
// 1. Get Stealth meta address from keys ✅
// 2. Generate new Stealth address ✅
// 3. Check stealth address fast ✅
// 5. Compute stealth key ✅

use eth_stealth_addresses::{
    check_stealth_address_fast, decode_priv, encode_pubkey, get_pubkey_from_priv,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

type Address = [u8; 20]; // 20 bytes
type StealthMetaAddress = [u8; 66];
type PublicKeyUncompressed = [u8; 65];
type PublicKeyCompressed = [u8; 33];
type PrivateKey = [u8; 32];

#[derive(Serialize, Deserialize)]
struct StealthAddressOutput {
    scheme_id: u8,
    address: String,
    ephemeral_pubkey: String,
    view_tag: u8,
}

#[wasm_bindgen]
pub fn new_stealth_address(stealth_meta_address: String) -> Result<JsValue, JsError> {
    let stealth_meta_address: StealthMetaAddress = unhexlify(&stealth_meta_address)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let (address, ephemeral_pubkey, view_tag) =
        eth_stealth_addresses::generate_stealth_address(&stealth_meta_address);

    let output = StealthAddressOutput {
        scheme_id: 0,
        address: hexlify(&address),
        ephemeral_pubkey: hexlify(&ephemeral_pubkey),
        view_tag,
    };

    Ok(serde_wasm_bindgen::to_value(&output)?)
}

#[wasm_bindgen]
pub fn get_stealth_meta_address(pks: String, pkv: String) -> Result<String, JsError> {
    let pks: PrivateKey = unhexlify(&pks)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;
    let pkv: PrivateKey = unhexlify(&pkv)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let stealth_meta_address = eth_stealth_addresses::encode_stealth_meta_address(
        get_pubkey_from_priv(decode_priv(&pks)),
        get_pubkey_from_priv(decode_priv(&pkv)),
    );

    let stealth_meta_address_hex = hexlify(&stealth_meta_address);
    Ok(stealth_meta_address_hex)
}

#[wasm_bindgen]
pub fn check_stealth(
    stealth_address: String,
    ephemeral_pubkey: String,
    viewing_key: String,
    spending_key: String,
    view_tag: u8,
) -> Result<bool, JsError> {
    let sk: PrivateKey = unhexlify(&spending_key)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let vk: PrivateKey = unhexlify(&viewing_key)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let spending_pubkey = encode_pubkey(get_pubkey_from_priv(decode_priv(&sk)));

    let stealth_address: Address = unhexlify(&stealth_address)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;
    let ephemeral_pubkey: PublicKeyCompressed = unhexlify(&ephemeral_pubkey)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let check = check_stealth_address_fast(
        &stealth_address,
        &ephemeral_pubkey,
        &vk,
        &spending_pubkey,
        view_tag,
    );
    Ok(check)
}

#[wasm_bindgen]
pub fn reveal_stealth_key(
    spending_key: String,
    viewing_key: String,
    stealth_address: String,
    ephemeral_pubkey: String,
) -> Result<String, JsError> {
    let sk: PrivateKey = unhexlify(&spending_key)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let vk: PrivateKey = unhexlify(&viewing_key)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let stealth_address: Address = unhexlify(&stealth_address)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let ephemeral_pubkey: PublicKeyCompressed = unhexlify(&ephemeral_pubkey)
        .as_slice()
        .try_into()
        .map_err(|_| JsError::new("Invalid value"))?;

    let stealth_key =
        eth_stealth_addresses::compute_stealth_key(&stealth_address, &ephemeral_pubkey, &vk, &sk);

    Ok(hexlify(&stealth_key))
}

// Optional
pub fn scan_for_payments() {
    // This function will first of all find all the announced stealth addresses by scanning all the events
    // Then it will check is the stealth address is indeed present for the current user
}

fn hexlify(a: &[u8]) -> String {
    let mut output = "0x".to_owned();
    output.push_str(&hex::encode(a));

    output
}

fn unhexlify(h: &String) -> Vec<u8> {
    let mut prefix = h.to_owned();
    let s = prefix.split_off(2);
    let result = hex::decode(&s);
    let out = match result {
        Ok(val) => val,
        Err(error) => panic!("error decoding hex: {:?}", error),
    };

    out
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
