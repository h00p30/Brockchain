//https://github.com/leeduckgo/secp256k1-example-rs/blob/main/src/main.rs

use bitcoin_hashes::{sha256, Hash};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{
    Error, Message, PublicKey, Secp256k1, SecretKey, Signature, Signing, Verification,
};

pub fn generate_keys() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().unwrap();
    let seckey = SecretKey::new(&mut rng);
    //println!("privkey: {:?}", seckey);
    let pubkey = PublicKey::from_secret_key(&secp, &seckey);
    //println!("pubkey: {:?}", pubkey);
    (seckey, pubkey)
}

pub fn sign(digest: &[u8], seckey: SecretKey) -> Signature {
    let secp = Secp256k1::new();
    let signature = do_sign(&secp, digest, seckey).unwrap();
    //println!("signature: {:?}", signature);
    signature
}

pub fn verify(digest: &[u8], sig: Signature, pubkey: PublicKey) -> bool {
    let secp = Secp256k1::new();
    do_verify(&secp, digest, sig, pubkey).unwrap()
}

pub fn do_sign<C: Signing>(
    secp: &Secp256k1<C>,
    digest: &[u8],
    seckey: SecretKey,
) -> Result<Signature, Error> {
    let digest = sha256::Hash::hash(digest);
    let digest = Message::from_slice(&digest)?;
    Ok(secp.sign(&digest, &seckey))
}

pub fn do_verify<C: Verification>(
    secp: &Secp256k1<C>,
    digest: &[u8],
    sig: Signature,
    pubkey: PublicKey,
) -> Result<bool, Error> {
    let digest = sha256::Hash::hash(digest);
    let digest = Message::from_slice(&digest)?;

    Ok(secp.verify(&digest, &sig, &pubkey).is_ok())
}
