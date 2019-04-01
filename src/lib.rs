#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Traits and functions to make writing proc macros more ergonomic.
//!
//! The *roids* name is chosen because, although these functions make it easy to perform certain
//! operation, they may not necessarily be a good idea =D!
//!
//! # Examples
//!
//! 1. Append additional `#[derive(..)]`s.
//!
//!    This works for function-like or attribute proc macros.
//!
//!     ```rust,edition2018
//!     use proc_macro_roids::DeriveInputDeriveExt;
//!     use syn::{parse_quote, DeriveInput};
//!
//!     # fn main() {
//!     // This may be parsed from the proc macro token stream.
//!     let mut ast: DeriveInput = parse_quote! {
//!         #[derive(Debug)]
//!         struct Struct;
//!     };
//!
//!     // Append the derives.
//!     let derives = parse_quote!(Clone, Copy);
//!     ast.append_derives(derives);
//!
//!     // That's it!
//!     let ast_expected: DeriveInput = parse_quote! {
//!         #[derive(Debug, Clone, Copy)]
//!         struct Struct;
//!     };
//!     assert_eq!(ast_expected, ast);
//!     # }
//!     ```
//!
//! 2. Append named fields.
//!
//!     This works for structs with named fields or unit structs.
//!
//!     ```rust,edition2018
//!     use proc_macro_roids::FieldsNamedAppend;
//!     use syn::{parse_quote, DeriveInput, FieldsNamed};
//!
//!     # fn main() {
//!     // This may be parsed from the proc macro token stream.
//!     let mut ast: DeriveInput = parse_quote! {
//!         struct StructNamed { a: u32, b: i32 }
//!     };
//!
//!     // Append the fields.
//!     let fields_additional: FieldsNamed = parse_quote!({ c: i64, d: usize });
//!     ast.append(fields_additional);
//!
//!     // That's it!
//!     let ast_expected: DeriveInput = parse_quote! {
//!         struct StructNamed { a: u32, b: i32, c: i64, d: usize }
//!     };
//!     assert_eq!(ast_expected, ast);
//!     # }
//!     ```
//!
//! 3. Append unnamed fields (tuples).
//!
//!     This works for structs with unnamed fields or unit structs.
//!
//!     ```rust,edition2018
//!     use proc_macro_roids::FieldsUnnamedAppend;
//!     use syn::{parse_quote, DeriveInput, FieldsUnnamed};
//!
//!     # fn main() {
//!     // This may be parsed from the proc macro token stream.
//!     let mut ast: DeriveInput = parse_quote! {
//!         struct StructUnit;
//!     };
//!
//!     // Append the fields.
//!     let fields_additional: FieldsUnnamed = parse_quote!((i64, usize));
//!     ast.append(fields_additional);
//!
//!     // That's it!
//!     let ast_expected: DeriveInput = parse_quote! {
//!         struct StructUnit(i64, usize);
//!     };
//!     assert_eq!(ast_expected, ast);
//!     # }
//!     ```
//!
//! 4. Get newtype inner `Field`.
//!
//!     This works for structs with unnamed fields or unit structs.
//!
//!     ```rust,edition2018
//!     use proc_macro_roids::DeriveInputNewtypeExt;
//!     use syn::{parse_quote, DeriveInput, Type};
//!
//!     # fn main() {
//!     // This may be parsed from the proc macro token stream.
//!     let mut ast: DeriveInput = parse_quote! {
//!         struct Newtype(u32);
//!     };
//!
//!     // Get the inner field.
//!     let inner_field = ast.inner_type_mut();
//!
//!     // That's it!
//!     let expected_type: Type = Type::Path(parse_quote!(u32));
//!     assert_eq!(expected_type, inner_field.ty);
//!     # }
//!     ```

pub use crate::{
    derive_input_derive_ext::DeriveInputDeriveExt,
    derive_input_newtype_ext::DeriveInputNewtypeExt,
    derive_input_struct_ext::DeriveInputStructExt,
    fields_named_append::FieldsNamedAppend,
    fields_unnamed_append::FieldsUnnamedAppend,
    util::{ident_concat, meta_list_contains, nested_meta_to_ident},
};

mod derive_input_derive_ext;
mod derive_input_newtype_ext;
mod derive_input_struct_ext;
mod fields_named_append;
mod fields_unnamed_append;
mod util;
