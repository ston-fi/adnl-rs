//! Integration with `curve25519_dalek` and `x25519_dalek` libraries

use crate::helper_types::AdnlPrivateKey;
use crate::{AdnlPublicKey, AdnlSecret};
use curve25519_dalek::edwards::CompressedEdwardsY;
use curve25519_dalek::montgomery::MontgomeryPoint;
use x25519_dalek::{PublicKey, StaticSecret};

impl AdnlPublicKey for PublicKey {
    fn to_bytes(&self) -> [u8; 32] {
        MontgomeryPoint(self.to_bytes())
            .to_edwards(0)
            .unwrap()
            .compress()
            .to_bytes()
    }
}

fn edwards_to_montgomery<P: AdnlPublicKey>(public_key: &P) -> PublicKey {
    PublicKey::from(
        CompressedEdwardsY::from_slice(&public_key.to_bytes())
            .unwrap()
            .decompress()
            .unwrap()
            .to_montgomery()
            .to_bytes(),
    )
}

impl AdnlPrivateKey for StaticSecret {
    type PublicKey = PublicKey;

    fn key_agreement<P: AdnlPublicKey>(&self, their_public: &P) -> AdnlSecret {
        AdnlSecret::from(
            self.diffie_hellman(&edwards_to_montgomery(their_public))
                .to_bytes(),
        )
    }

    fn public(&self) -> Self::PublicKey {
        PublicKey::from(self)
    }
}
