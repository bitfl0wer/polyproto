// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::signature::Signature;

use der::Encode;
use spki::AlgorithmIdentifierOwned;

/// A cryptographic private key generated by a [AlgorithmIdentifierOwned], with
/// a corresponding [PublicKey]
pub trait PrivateKey<S: Signature>: PartialEq + Eq + Encode {
    type PublicKey: PublicKey<S>;
    /// Returns a shared reference to [Self::PrivateKey]
    fn as_ref(&self) -> &Self;
    /// Returns an exclusive reference to [Self::PrivateKey]
    fn as_mut(&mut self) -> &mut Self;
    /// Returns the public key corresponding to this private key.
    fn pubkey(&self) -> &Self::PublicKey;
    /// Creates a [Signature] for the given data.
    fn sign(&self, data: &[u8]) -> S;
    /// Returns the [AlgorithmIdentifierOwned] used for this key.
    fn algorithm(&self) -> AlgorithmIdentifierOwned;
}

/// A cryptographic public key generated by a [SignatureAlgorithm].
pub trait PublicKey<S: Signature>: PartialEq + Eq + Encode {
    type Error;
    /// Returns a shared reference to [Self::PublicKey]
    fn as_ref(&self) -> &Self;
    /// Returns an exclusive reference to [Self::PublicKey]
    fn as_mut(&mut self) -> &mut Self;
    /// Verifies the correctness of a given [Signature] for a given piece of data.
    ///
    /// Implementations of this associated method should mitigate against signature malleability
    fn verify_signature(&self, signature: &S, data: &[u8]) -> Result<(), Self::Error>;
    /// Returns the [AlgorithmIdentifierOwned] used for this key.
    fn algorithm(&self) -> AlgorithmIdentifierOwned;
}
