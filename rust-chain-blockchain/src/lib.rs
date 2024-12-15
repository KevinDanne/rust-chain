pub mod block;
pub mod block_chain;
pub mod transaction;
pub mod wallet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut chain = block_chain::BlockChain::new();

        let wallet_alice = wallet::Wallet::new().unwrap();
        let wallet_bob = wallet::Wallet::new().unwrap();

        let mut first_block = block::Block::new(None);
        first_block.add_transaction(wallet_alice.create_transaction(&wallet_bob, 100.0).unwrap());
        first_block.add_transaction(wallet_bob.create_transaction(&wallet_alice, 5.0).unwrap());
        first_block.add_transaction(wallet_alice.create_transaction(&wallet_bob, 10.0).unwrap());
        chain.add_block(&first_block);

        let mut second_block = block::Block::new(Some(first_block.id().clone()));
        second_block.add_transaction(wallet_bob.create_transaction(&wallet_alice, 20.0).unwrap());
        chain.add_block(&second_block);

        println!("Iterating blocks of blockchain with id {}", chain.id());
        for block in chain.blocks().iter() {
            println!(
                "Iterating transactions of block with id {} | valid: {}",
                block.id(),
                block.verify()
            );
            for trans in block.transactions() {
                println!(
                    "Transaction {} with amount {}RCC",
                    trans.id(),
                    trans.amount()
                );
            }
        }
        println!(
            "Verify result for blockchain with id {}: {}",
            chain.id(),
            chain.verify()
        );
    }
}
