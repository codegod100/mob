use anyhow::{Error, Result};
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

pub struct KeyPair {
    signer: Option<SigningKey>,
}
impl KeyPair {
    /// Generate a new keypair with a random key
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = SigningKey::generate(&mut csprng);
        Self {
            signer: Some(keypair),
        }
    }

    /// Generate a new keypair with the provided b64 encoded key
    pub fn new_with_key(key: &str) -> Result<Self> {
        let b = BASE64_STANDARD.decode(key)?;
        let b: [u8; 32] = match b.try_into() {
            Ok(v) => v,
            Err(_) => return Err(Error::msg("failed to cast bytes")),
        };

        let sk = SigningKey::from_bytes(&b);
        let s = Self { signer: Some(sk) };
        Ok(s)
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
            Err(_) => return Err(Error::msg("failed to cast bytes")),
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

    pub fn import(&mut self, key: String) -> Result<()> {
        let b = BASE64_STANDARD.decode(key)?;
        let b: [u8; 32] = match b.try_into() {
            Ok(v) => v,
            Err(_) => return Err(Error::msg("failed to cast bytes")),
        };

        let sk = SigningKey::from_bytes(&b);
        self.signer = Some(sk);
        Ok(())
    }

    pub fn reset(&mut self) -> () {
        self.signer = KeyPair::new().signer;
    }

    pub fn export(&self) -> Result<String> {
        let signer = self.signer()?;
        let b64 = BASE64_STANDARD.encode(signer.to_bytes());
        Ok(b64)
    }

    fn signer(&self) -> Result<SigningKey> {
        match &self.signer {
            Some(signer) => Ok(signer.clone()),
            None => Err(Error::msg("missing signing key")),
        }
    }
    pub fn public_key(&self) -> Result<String> {
        let signer = self.signer()?;
        let v = signer.verifying_key();
        let b64 = BASE64_STANDARD.encode(v.to_bytes());
        Ok(b64)
    }
}
