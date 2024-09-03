use anyhow::Result;
use base64::prelude::*;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[derive(Default, Debug)]
pub struct KeyPair {
    signer: Option<SigningKey>,
}
impl KeyPair {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = SigningKey::generate(&mut csprng);
        Self {
            signer: Some(keypair),
        }
    }
    pub fn sign(&self, message: &str) -> Result<String> {
        let sig = match &self.signer {
            Some(signer) => signer.sign(message.as_bytes()),
            None => return Err(anyhow::Error::msg("missing signing key")),
        };
        let b64 = BASE64_STANDARD.encode(sig.to_bytes());
        Ok(b64)
    }
    pub fn verify(&self, message: &str, signature: &str) -> Result<bool> {
        let message = message.as_bytes();
        let b64 = BASE64_STANDARD.decode(signature)?;
        let b: &[u8] = &b64;
        let sig = Signature::from_slice(b)?;
        let v = match &self.signer {
            Some(signer) => signer.verify(message, &sig),
            None => return Err(anyhow::Error::msg("missing signing key")),
        };
        match v {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn save(&self, path: Option<&str>) {
        println!("Saving {:#?} to {}", self.signer, path.unwrap_or_default());
    }
}
