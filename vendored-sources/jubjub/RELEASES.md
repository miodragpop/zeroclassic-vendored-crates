# 0.5.1

# Fixed
* The crate now compiles for non-64-bit targets, such as the `wasm32-*` targets.

# 0.5.0

This upgrade bumps our dependencies `bls12_381`, `group` and `ff`, while making
corresponding changes to the APIs. This release now only supports Rust compilers
version 1.44.0 or later.

# 0.4.0

This release adds implementations of the `ff` and `group` traits. Additional trait
implementations (for standard traits) have been added where the `ff` and `group` trait
bounds require them.

## Added
* `jubjub::SubgroupPoint`, which represents an element of Jubjub's prime-order subgroup.
  It implements the following traits:
  * `group::{Group, GroupEncoding}`
  * `group::prime::PrimeGroup`
* New trait implementations for `jubjub::ExtendedPoint`:
  * `group::{Curve, Group, GroupEncoding, WnafGroup}`
  * `group::cofactor::{CofactorCurve, CofactorGroup}`
* New trait implementations for `jubjub::AffinePoint`:
  * `group::GroupEncoding`
  * `group::cofactor::CofactorCurveAffine`
* New trait implementations for `jubjub::Fr`:
  * `ff::{Field, PrimeField}`
* `jubjub::AffinePoint::is_identity`
* `jubjub::AffinePoint::to_extended`
* `jubjub::Scalar`, as an alias for `jubjub::Fr`.

## Changed
* We've migrated to `bls12_381 0.2`.
* `rand_core` is now a regular dependency.
* We depend on the `byteorder` crate again, as it is part of the `ff::PrimeField` trait.
* The benchmarks are now implemented using `criterion`.

# 0.3.0

This release now depends on the `bls12_381` crate, which exposes the `Fq` field type that we re-export.

* The `Fq` and `Fr` field types now have better constant function support for various operations and constructors.
* We no longer depend on the `byteorder` crate.
* We've bumped our `rand_core` dev-dependency up to 0.5.
* We've removed the `std` and `nightly` features.
* We've bumped our dependency of `subtle` up to `^2.2.1`.

# 0.2.0

This release switches to `subtle 2.1` to bring in the `CtOption` type, and also makes a few useful API changes.

* Implemented `Mul<Fr>` for `AffineNielsPoint` and `ExtendedNielsPoint`
* Changed `AffinePoint::to_niels()` to be a `const` function so that constant curve points can be constructed without statics.
* Implemented `multiply_bits` for `AffineNielsPoint`, `ExtendedNielsPoint`
* Removed `CtOption` and replaced it with `CtOption` from `subtle` crate.
* Modified receivers of some methods to reduce stack usage
* Changed various `into_bytes` methods into `to_bytes`

# 0.1.0

Initial release.
