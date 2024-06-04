// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use der::{Any, Decode, Encode};
use spki::ObjectIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
/// `AlgorithmIdentifier` reference which has `Any` parameters.
///
/// A wrapper around `spki::AlgorithmIdentifierOwned`, which provides `serde` support, if enabled by
/// the `serde` feature.
///
/// ## De-/Serialization expectations
///
/// This type expects a DER encoded AlgorithmIdentifier with optional der::Any parameters. The DER
/// encoded data has to be provided in the form of an array of bytes. Types that fulfill this
/// expectation are, for example, `&[u8]`, `Vec<u8>` and `&[u8; N]`.
pub struct AlgorithmIdentifierOwned(spki::AlgorithmIdentifierOwned);

impl AlgorithmIdentifierOwned {
    pub fn new(oid: ObjectIdentifier, parameters: Option<Any>) -> Self {
        Self(spki::AlgorithmIdentifierOwned { oid, parameters })
    }

    /// Try to encode this type as DER.
    pub fn to_der(&self) -> Result<Vec<u8>, der::Error> {
        self.0.to_der()
    }

    /// Try to decode this type from DER.
    pub fn from_der(bytes: &[u8]) -> Result<AlgorithmIdentifierOwned, der::Error> {
        spki::AlgorithmIdentifierOwned::from_der(bytes).map(Self)
    }
}

impl Deref for AlgorithmIdentifierOwned {
    type Target = spki::AlgorithmIdentifierOwned;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AlgorithmIdentifierOwned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<spki::AlgorithmIdentifierOwned> for AlgorithmIdentifierOwned {
    fn from(value: spki::AlgorithmIdentifierOwned) -> Self {
        Self(value)
    }
}

impl From<AlgorithmIdentifierOwned> for spki::AlgorithmIdentifierOwned {
    fn from(value: AlgorithmIdentifierOwned) -> Self {
        value.0
    }
}

#[cfg(feature = "serde")]
mod serde_support {
    use super::AlgorithmIdentifierOwned;
    use serde::de::Visitor;
    use serde::{Deserialize, Serialize};
    struct AlgorithmIdentifierVisitor;

    impl<'de> Visitor<'de> for AlgorithmIdentifierVisitor {
        type Value = AlgorithmIdentifierOwned;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a DER encoded AlgorithmIdentifier with optional der::Any parameters and a BitString Key")
        }
    }

    impl<'de> Deserialize<'de> for AlgorithmIdentifierOwned {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_bytes(AlgorithmIdentifierVisitor)
        }
    }

    impl Serialize for AlgorithmIdentifierOwned {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let der = self.to_der().map_err(serde::ser::Error::custom)?;
            serializer.serialize_bytes(&der)
        }
    }
}

// TODO: Tests
