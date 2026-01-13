use strata::encode::encode;
use strata::error::EncodeError;
use strata::hash::hash_value;
use strata::value::Value;

/// Output of hashing a Strata value for storage as a package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedPackage {
    /// BLAKE3-256 hash of the Strata value over canonical bytes.
    pub hash: Vec<u8>,
    /// Strata Canonical Bytes of the value.
    pub scb: Vec<u8>,
}

/// Computes (hash, scb) for a Strata value.
///
/// IMPORTANT:
/// - `hash` must match Strata's `hash_value` semantics.
/// - `scb` must be the canonical encoding of the same value.
///
/// This module is the only place in LogiCore that should know Strata hashing details.
///
/// # Errors
/// Returns `EncodeError` if the value cannot be canonically encoded.
pub fn hash_strata_value(value: &Value) -> Result<HashedPackage, EncodeError> {
    let scb: Vec<u8> = encode(value)?;

    // Strata hashes are computed over canonical encoded bytes.
    // `strata::hash::hash_value` implements the canonical hash semantics.
    let hash: [u8; 32] = hash_value(value);

    Ok(HashedPackage {
        hash: hash.to_vec(),
        scb,
    })
}
