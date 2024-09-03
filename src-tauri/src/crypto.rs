use anyhow::Result;
use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signer;
use rand::rngs::OsRng;

#[derive(Default, Debug)]
pub struct KeyPair {
    signer: Option<SigningKey>,
}
impl KeyPair {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = SigningKey::generate(&mut csprng);
        Self { signer: Some(keypair) }
    }
    pub fn sign(&self, message: &str) -> Result<Signature> {
        match &self.signer {
            Some(signer) => Ok(signer.sign(message.as_bytes())),
            None => Err(anyhow::Error::msg("missing signing key")),
        }
    }
    pub fn save(&self, path: Option<&str>) {
        println!("Saving {:#?} to {}", self.signer, path.unwrap_or_default());
    }
}
