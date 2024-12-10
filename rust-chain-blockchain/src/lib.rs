pub mod wallet;
pub mod transaction;
pub mod block;
pub mod block_chain;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut chain = block_chain::BlockChain::new(); 

        let wallet_kevin = wallet::Wallet::new().unwrap();
        let wallet_louis = wallet::Wallet::new().unwrap();

        let mut first_block = block::Block::new(None);
        first_block.add_transaction(wallet_kevin.create_transaction(wallet_louis.public_key(), 100_f64).unwrap());
        chain.add_block(first_block);

        let previous_id = chain.blocks().last().unwrap().id().clone();
        let mut second_block = block::Block::new(Some(previous_id));
        chain.add_block(second_block);

        for block in chain.blocks() {
            println!("Iterating transactions of block {}", block.id());
            for trans in block.transactions() {
                println!("Transaction {} with amount {}RCC", trans.id(), trans.amount());
            }
        }
    }
}