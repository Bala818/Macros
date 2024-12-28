mod balances;
mod macros;
mod proof_of_existence;

use balances::Balances;
use macros::{call, runtime};
use proof_of_existence::ProofOfExistence;

runtime!(Balances, ProofOfExistence);
