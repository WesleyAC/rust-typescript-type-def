//! # TypeScript Type Definitions for Rust Types
//!
//! This crate allows you to produce a TypeScript module containing type
//! definitions which describe the JSON serialization of Rust types. The
//! intended use is to define TypeScript types for data that is serialized from
//! Rust types as JSON using [`serde_json`](https://docs.rs/serde_json/) so it
//! can be safely used from TypeScript without needing to maintain a parallel
//! set of type definitions.
//!
//! ## Examples
//!
//! Simple example:
//! ```
//! use serde::Serialize;
//! use typescript_type_def::{write_definition_file, TypeDef};
//!
//! #[derive(Serialize, TypeDef)]
//! struct Foo {
//!     a: usize,
//!     b: String,
//! }
//!
//! let ts_module = {
//!     let mut buf = Vec::new();
//!     write_definition_file::<_, Foo>(&mut buf, Default::default()).unwrap();
//!     String::from_utf8(buf).unwrap()
//! };
//! assert_eq!(
//!     ts_module,
//!     r#"// AUTO-GENERATED by typescript-type-def
//!
//! export default types;
//! export namespace types{
//! export type Usize=number;
//! export type Foo={"a":types.Usize;"b":string;};
//! }
//! "#
//! );
//!
//! let foo = Foo {
//!     a: 123,
//!     b: "hello".to_owned(),
//! };
//! let json = serde_json::to_string(&foo).unwrap();
//! // This JSON matches the TypeScript type definition above
//! assert_eq!(json, r#"{"a":123,"b":"hello"}"#);
//! ```
//!
//! When working with a large codebase consisting of many types, a useful
//! pattern is to declare an "API" type alias which lists all the types you want
//! to make definitions for. For example:
//! ```
//! use serde::Serialize;
//! use typescript_type_def::{write_definition_file, TypeDef};
//!
//! #[derive(Serialize, TypeDef)]
//! struct Foo {
//!     a: String,
//! }
//!
//! #[derive(Serialize, TypeDef)]
//! struct Bar {
//!     a: String,
//! }
//!
//! #[derive(Serialize, TypeDef)]
//! struct Baz {
//!     a: Qux,
//! }
//!
//! #[derive(Serialize, TypeDef)]
//! struct Qux {
//!     a: String,
//! }
//!
//! // This type lists all the top-level types we want to make definitions for.
//! // You don't need to list *every* type in your API here, only ones that
//! // wouldn't be referenced otherwise. Note that `Qux` is not mentioned, but
//! // is still emitted because it is a dependency of `Baz`.
//! type Api = (Foo, Bar, Baz);
//!
//! let ts_module = {
//!     let mut buf = Vec::new();
//!     write_definition_file::<_, Api>(&mut buf, Default::default()).unwrap();
//!     String::from_utf8(buf).unwrap()
//! };
//! assert_eq!(
//!     ts_module,
//!     r#"// AUTO-GENERATED by typescript-type-def
//!
//! export default types;
//! export namespace types{
//! export type Foo={"a":string;};
//! export type Bar={"a":string;};
//! export type Qux={"a":string;};
//! export type Baz={"a":types.Qux;};
//! }
//! "#
//! );
//! ```
#![warn(rust_2018_idioms, clippy::all, missing_docs)]
#![deny(clippy::correctness)]

mod emit;
mod impls;
pub mod type_expr;

pub use crate::{
    emit::{
        write_definition_file,
        DefinitionFileOptions,
        Deps,
        Stats,
        TypeDef,
    },
    impls::Blob,
};

/// A derive proc-macro for the [`TypeDef`] trait.
///
/// This macro can be used on `struct`s and `enum`s which also derive
/// [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html)
/// and/or
/// [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html),
/// and will generate a [`TypeDef`] implementation which matches the shape
/// of the JSON produced by using [`serde_json`](https://docs.rs/serde_json/) on
/// the target type. This macro will also read and adapt to `#[serde(...)]`
/// attributes on the target type's definition.
///
/// This macro also reads the following attributes:
/// * `#[type_def(namespace = "x.y.z")]` on the `struct`/`enum` body puts
///   the TypeScript type definition under a namespace of `x.y.z`. Note
///   that [`write_definition_file`] will additionally place all type
///   definitions under a namespace called `types`.
// TODO: add description of what shapes are generated for various types
// newtypes, enums, optional struct fields, etc.
pub use typescript_type_def_derive::TypeDef;
