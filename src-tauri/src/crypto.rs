use std::io::Bytes;
use std::io::Read;

use anyhow::Result;
use base64::prelude::*;
use ed25519_dalek::ed25519::signature::Keypair;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Verifier;
use ed25519_dalek::VerifyingKey;
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
        let signer = self.signer()?;
        let sig = signer.sign(message.as_bytes());

        let b64 = BASE64_STANDARD.encode(sig.to_bytes());
        Ok(b64)
    }
    pub fn verify(&self, public_key: &str, signature: &str, message: &str) -> Result<bool> {
        let vk = BASE64_STANDARD.decode(public_key)?;
        let vk: [u8; 32] = match vk.try_into() {
            Ok(v) => v,
            Err(_) => return Err(anyhow::Error::msg("failed to cast bytes")),
        };
        let vk = VerifyingKey::from_bytes(&vk)?;
        let message = message.as_bytes();
        let b64 = BASE64_STANDARD.decode(signature)?;
        let b: &[u8] = &b64;
        let sig = Signature::from_slice(b)?;
        let v = vk.verify(message, &sig);

        match v {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn save(&self, path: Option<&str>) {
        println!("Saving {:#?} to {}", self.signer, path.unwrap_or_default());
    }

    fn signer(&self) -> Result<SigningKey> {
        match &self.signer {
            Some(signer) => Ok(signer.clone()),
            None => Err(anyhow::Error::msg("missing signing key")),
        }
    }
    pub fn public_key(&self) -> Result<String> {
        let signer = self.signer()?;
        let v = signer.verifying_key();
        let b64 = BASE64_STANDARD.encode(v.to_bytes());
        Ok(b64)
    }
}
