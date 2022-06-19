mod block;
mod blockchain;
mod keygenerator;
use crate::blockchain::block::transaction::Transaction;

fn main() {
    //Generate address which sends reward
    let (reward_seckey, reward_pubkey) = keygenerator::generate_keys();
    //Create blockchain
    let mut blockchain =
        blockchain::Blockchain::new(3, 100, reward_pubkey).expect("Could not create blockchain");
    //Generate some keys
    let (seckey, pubkey) = keygenerator::generate_keys();
    let (seckey2, pubkey2) = keygenerator::generate_keys();
    //Create then sign transaction
    let mut tx = Transaction::new(pubkey, pubkey2, 100).expect("Could not create transaction");
    tx.sign_transaction(pubkey2, seckey2);
    //Send then mine transaction
    blockchain.add_transaction(tx);
    blockchain.mine_pending_transactions(pubkey);

    println!("{}", blockchain.is_chain_valid());
    println!("{}", blockchain.get_balance_of(pubkey));

    blockchain.mine_pending_transactions(pubkey);
    //Miner gets paid
    println!("{}", blockchain.get_balance_of(pubkey));
}
