use crate::call;

pub struct Balances;

impl Balances {
    pub fn transfer(from: String, to: String, amount: u128) -> Result<(), String> {
        if amount == 0 {
            Err("Amount cannot be zero.".to_string())
        } else {
            println!("Transferred {} from {} to {}.", amount, from, to);
            Ok(())
        }
    }
}

call!(Balances, transfer(from: String, to: String, amount: u128) -> Result<(), String>);
