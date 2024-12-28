use crate::call;

pub struct ProofOfExistence;

impl ProofOfExistence {
    pub fn create_claim(owner: String, content: String) -> Result<(), String> {
        println!("Claim created by {} for content: {}", owner, content);
        Ok(())
    }

    pub fn revoke_claim(owner: String, content: String) -> Result<(), String> {
        println!("Claim revoked by {} for content: {}", owner, content);
        Ok(())
    }
}

call!(
    ProofOfExistence,
    create_claim(owner: String, content: String) -> Result<(), String>,
    revoke_claim(owner: String, content: String) -> Result<(), String>
);
