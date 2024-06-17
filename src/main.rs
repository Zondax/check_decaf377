use decaf377::Fr;
use decaf377_rdsa::{SigningKey, SpendAuth, VerificationKey};

// Secret key in hex format
const SPEND_KEY: &str = "ff726c71bcec76abc6a88cba71df655b28de6580edbd33c7415fdfded2e422e7";

pub fn expand(label: &'static [u8; 16], key: &[u8], input: &[u8]) -> [u8; 64] {
    let hash = blake2b_simd::Params::new()
        .personal(label)
        .key(key)
        .hash(input);

    hash.as_bytes().try_into().unwrap()
}

pub fn expand_ff(label: &'static [u8; 16], key: &[u8], input: &[u8]) -> Fr {
    Fr::from_le_bytes_mod_order(expand(label, key, input).as_ref())
}

fn main() {
    // use generated secret key bytes to generate a verification key
    let key = hex::decode(SPEND_KEY).unwrap();

    // We reproduce how a verification key is created from raw key bytes
    let fr = expand_ff(b"Penumbra_ExpndSd", &key, &[0; 1]);
    let ask = SigningKey::new_from_field(fr);
    let vk: VerificationKey<SpendAuth> = ask.into();

    println!("Verification key: {:?}", vk.to_bytes());
}
