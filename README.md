[![Crates.io Version](https://img.shields.io/crates/v/type-sig)](https://crates.io/crates/type-sig) [![docs.rs](https://img.shields.io/docsrs/type-sig)](https://docs.rs/type-sig/)
This crate allows you to get a recursive type signature of a provided type.

Example:
```rs
type_sig::<u8>() // TypeSignature { name: "u8", id: TypeId(..), .. }
type_sig::<*const *mut u8>() // TypeSignature { name: "*const *mut u8", .., children: [TypeSignature { name: "*mut u8", .. }] }
```
