// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!

Generic rust types and traits to quickly get a
[polyproto](https://docs.polyphony.chat/Protocol%20Specifications/core/) implementation up and
running.

## Implementing polyproto

Start by implementing the traits in [`crate::signature`] and [`crate::key`]. You can then
use the [`crate::cert`] types to build certificates using your implementations of the
aformentioned traits.

## Cryptography

This crate provides no cryptographic functionality whatsoever; its sole purpose is to aid in
implementing polyproto by transforming the
[polyproto specification](https://docs.polyphony.chat/Protocol%20Specifications/core/) into
well-defined yet adaptable Rust types.

*/

#[deny(missing_docs)]

/// Generic polyproto certificate types and traits.
pub mod cert;
/// Generic polyproto public- and private key traits.
pub mod key;
/// Generic polyproto signature traits.
pub mod signature;

use std::fmt::Debug;

use thiserror::Error;

/// Error type covering possible failures when converting a [`x509_cert::TbsCertificate`]
/// to a [`crate::cert::IdCertTbs`]
#[derive(Error, Debug, PartialEq)]
pub enum TbsCertToIdCert {
    #[error("field 'subject_unique_id' was None. Expected: Some(der::asn1::BitString)")]
    SubjectUid,
    #[error("field 'extensions' was None. Expected: Some(x509_cert::ext::Extensions)")]
    Extensions,
    #[error("Supplied integer too long")]
    Signature(der::Error),
}

/// Error type covering possible failures when converting a [`crate::cert::IdCertTbs`]
/// to a [`x509_cert::TbsCertificate`]
#[derive(Error, Debug, PartialEq)]
pub enum IdCertToTbsCert {
    #[error("Serial number could not be converted")]
    SerialNumber(der::Error),
}

#[cfg(test)]
mod test {
    use der::asn1::Uint;
    use x509_cert::certificate::Profile;
    use x509_cert::serial_number::SerialNumber;

    #[derive(Clone, PartialEq, Eq, Debug)]
    enum TestProfile {}

    impl Profile for TestProfile {}

    fn strip_leading_zeroes(bytes: &[u8]) -> &[u8] {
        if let Some(stripped) = bytes.strip_prefix(&[0u8]) {
            stripped
        } else {
            bytes
        }
    }

    #[test]
    fn test_convert_serial_number() {
        let biguint = Uint::new(&[10u8, 240u8]).unwrap();
        assert_eq!(biguint.as_bytes(), &[10u8, 240u8]);
        let serial_number: SerialNumber<TestProfile> =
            SerialNumber::new(biguint.as_bytes()).unwrap();
        assert_eq!(
            strip_leading_zeroes(serial_number.as_bytes()),
            biguint.as_bytes()
        );

        let biguint = Uint::new(&[240u8, 10u8]).unwrap();
        assert_eq!(biguint.as_bytes(), &[240u8, 10u8]);
        let serial_number: SerialNumber<TestProfile> =
            SerialNumber::new(biguint.as_bytes()).unwrap();
        assert_eq!(
            strip_leading_zeroes(serial_number.as_bytes()),
            biguint.as_bytes()
        );
    }
}
