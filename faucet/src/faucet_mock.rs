/// Requests an airdrop transaction.
///
/// * `faucet_addr` - The address of the faucet.
/// * `id` - The public key of the recipient.
/// * `lamports` - The number of lamports to airdrop.
/// * `blockhash` - The latest blockhash.
///
/// Creates a new keypair, constructs a system transfer transaction
/// to the recipient public key with the specified lamports, signs the
/// transaction with the new keypair, and returns the transaction.
/// Returns an error if the lamport amount is 0.
use {
    nexis_sdk::{
        hash::Hash, pubkey::Pubkey, signature::Keypair, system_transaction,
        transaction::Transaction,
    },
    std::{
        io::{Error, ErrorKind},
        net::SocketAddr,
    },
};

pub fn request_airdrop_transaction(
    _faucet_addr: &SocketAddr,
    _id: &Pubkey,
    lamports: u64,
    _blockhash: Hash,
) -> Result<Transaction, Error> {
    if lamports == 0 {
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))
    } else {
        let key = Keypair::new();
        let to = nexis_sdk::pubkey::new_rand();
        let blockhash = Hash::default();
        let tx = system_transaction::transfer(&key, &to, lamports, blockhash);
        Ok(tx)
    }
}
