mod balances;
mod macros;
mod proof_of_existence;
mod runtime;

use runtime::Call;
use runtime::Runtime;

fn main() {
    // Dispatch call to Balances
    Runtime::dispatch(Call::Balances(balances::Call::transfer {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 100,
    }))
    .unwrap();

    // Dispatch call to ProofOfExistence
    Runtime::dispatch(Call::ProofOfExistence(
        proof_of_existence::Call::create_claim {
            owner: "Alice".to_string(),
            content: "Document1".to_string(),
        },
    ))
    .unwrap();
}
