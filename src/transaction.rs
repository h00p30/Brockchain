extern crate sha2;

use secp256k1::{PublicKey, SecretKey, Signature};
use sha2::{Digest, Sha256};

#[path = "keygenerator.rs"]
pub mod keygenerator;

#[derive(Debug)]
pub struct Transaction {
    pub to: PublicKey,
    pub from: PublicKey,
    pub amount: usize,
    pub signature: Option<Signature>,
}

impl Transaction {
    pub fn new(
        to: PublicKey,
        from: PublicKey,
        amount: usize,
    ) -> Result<Transaction, std::io::Error> {
        let tx = Transaction {
            to,
            from,
            amount,
            signature: None,
        };
        Ok(tx)
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.to, self.from, self.amount));
        let result = hasher.finalize();
        let string = format!("{:x}", result);
        string
    }

    pub fn sign_transaction(&mut self, pubkey: PublicKey, seckey: SecretKey) {
        assert!(pubkey == self.from);

        let hash = self.calculate_hash();
        let sig = keygenerator::sign(hash.as_bytes(), seckey);
        self.signature = Some(sig);
    }

    pub fn is_valid(&self, reward_pubkey: PublicKey) -> bool {
        if self.from == reward_pubkey {
            return true;
        } else if self.signature == None {
            return false;
        } else {
            return keygenerator::verify(
                self.calculate_hash().as_bytes(),
                self.signature.unwrap(),
                self.from,
            );
        }
    }
}
