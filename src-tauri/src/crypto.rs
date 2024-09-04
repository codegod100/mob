use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

use anyhow::{Error, Result};
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
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
        let mut file = File::create("secret.txt")?;
        file.write_all(&b)?;
        let b: [u8; 32] = match b.try_into() {
            Ok(v) => v,
            Err(_) => return Err(Error::msg("failed to cast bytes")),
        };
        let sk = SigningKey::from_bytes(&b);
        self.signer = Some(sk);
        Ok(())
    }
    pub fn load() -> Result<Self> {
        let mut buffer = [0; 32];
        let mut file = File::open("secret.txt")?;
        let br = file.read(&mut buffer[..])?;
        println!("bytes read: {:#?}", br);
        if br == 0 {
            return Err(Error::msg("file empty"));
        }
        let sk = SigningKey::from_bytes(&buffer);
        let s = Self { signer: Some(sk) };
        Ok(s)
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
