#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Traits and functions to make writing proc macros more ergonomic.
//!
//! The *roids* name is chosen because, although these functions make it easy to perform certain
//! operation, they may not necessarily be a good idea =D!
//!
//! Makes writing procedural macros much easier:
//!
//! ```rust,ignore
//! extern crate proc_macro;
//!
//! use proc_macro::TokenStream;
//! use proc_macro2::Span;
//! use proc_macro_roids::{DeriveInputStructExt, FieldExt, IdentExt};
//! use quote::quote;
//! use syn::{parse_macro_input, DeriveInput, Ident};
//!
//! /// Derives a `Super` enum with a variant for each struct field:
//! ///
//! /// ```rust,edition2018
//! /// use std::marker::PhantomData;
//! /// use super_derive::Super;
//! ///
//! /// #[derive(Super)]
//! /// pub struct Man<T> {
//! ///     #[super_derive(skip)]
//! ///     name: String,
//! ///     power_level: u64,
//! ///     marker: PhantomData<T>,
//! /// }
//! /// ```
//! ///
//! /// Generates:
//! ///
//! /// ```rust,ignore
//! /// pub enum SuperMan {
//! ///     U64(u64),
//! /// }
//! /// ```
//! #[proc_macro_derive(Super, attributes(super_derive))]
//! pub fn system_desc_derive(input: TokenStream) -> TokenStream {
//!     let ast = parse_macro_input!(input as DeriveInput);
//!     let enum_name = ast.ident.prepend("Super");
//!     let fields = ast.fields();
//!     let relevant_fields = fields
//!         .iter()
//!         .filter(|field| !field.is_phantom_data())
//!         .filter(|field| !field.contains_tag("super_derive", "skip"));
//!
//!     let variants = relevant_fields
//!         .map(|field| {
//!             let type_name = field.type_name();
//!             let variant_name = type_name.to_string().to_uppercase();
//!             let variant_name = Ident::new(&variant_name, Span::call_site());
//!             quote! {
//!                 #variant_name(#type_name)
//!             }
//!         })
//!         .collect::<Vec<_>>();
//!
//!     let token_stream2 = quote! {
//!         pub enum #enum_name {
//!             #(#variants,)*
//!         }
//!     };
//!
//!     token_stream2.into()
//! }
//! ```
//!
//! # Examples
//!
//! 1. Append additional `#[derive(..)]`s.
//!
//!    This works for function-like or attribute proc macros.
//!
//!     ```rust,ignore
//!     extern crate proc_macro;
//!
//!     use proc_macro::TokenStream;
//!     use proc_macro_roids::DeriveInputExt;
//!     use quote::quote;
//!     use syn::{parse_macro_input, parse_quote, DeriveInput};
//!
//!     #[proc_macro_attribute]
//!     pub fn copy(_args: TokenStream, item: TokenStream) -> TokenStream {
//!         // Example input:
//!         //
//!         // #[derive(Debug)]
//!         // struct Struct;
//!         let mut ast = parse_macro_input!(item as DeriveInput);
//!
//!         // Append the derives.
//!         let derives = parse_quote!(Clone, Copy);
//!         ast.append_derives(derives);
//!
//!         // Example output:
//!         //
//!         // #[derive(Debug, Clone, Copy)]
//!         // struct Struct;
//!         TokenStream::from(quote! { #ast })
//!     }
//!     ```
//!
//! 2. Append named fields.
//!
//!     This works for structs with named fields or unit structs.
//!
//!     ```rust,ignore
//!     extern crate proc_macro;
//!
//!     use proc_macro::TokenStream;
//!     use proc_macro_roids::FieldsNamedAppend;
//!     use quote::quote;
//!     use syn::{parse_macro_input, parse_quote, DeriveInput, FieldsNamed};
//!
//!     /// Example usage:
//!     ///
//!     /// ```rust
//!     /// use macro_crate::append_cd;
//!     ///
//!     /// #[append_cd]
//!     /// struct StructNamed { a: u32, b: i32 }
//!     /// ```
//!     #[proc_macro_attribute]
//!     pub fn append_cd(_args: TokenStream, item: TokenStream) -> TokenStream {
//!         // Example input:
//!         //
//!         // struct StructNamed { a: u32, b: i32 }
//!         let mut ast = parse_macro_input!(item as DeriveInput);
//!
//!         // Append the fields.
//!         let fields_additional: FieldsNamed = parse_quote!({ c: i64, d: usize });
//!         ast.append_named(fields_additional);
//!
//!         // Example output:
//!         //
//!         // struct StructNamed { a: u32, b: i32, c: i64, d: usize }
//!         TokenStream::from(quote! { #ast })
//!     }
//!     ```
//!
//! 3. Append unnamed fields (tuples).
//!
//!     This works for structs with unnamed fields or unit structs.
//!
//!     ```rust,ignore
//!     extern crate proc_macro;
//!
//!     use proc_macro::TokenStream;
//!     use proc_macro_roids::FieldsUnnamedAppend;
//!     use quote::quote;
//!     use syn::{parse_macro_input, parse_quote, DeriveInput, FieldsUnnamed};
//!
//!     /// Example usage:
//!     ///
//!     /// ```rust
//!     /// use macro_crate::append_i64_usize;
//!     ///
//!     /// #[append_i64_usize]
//!     /// struct StructUnit;
//!     /// ```
//!     #[proc_macro_attribute]
//!     pub fn append_i64_usize(_args: TokenStream, item: TokenStream) -> TokenStream {
//!         // Example input:
//!         //
//!         // struct StructUnit;
//!         let mut ast = parse_macro_input!(item as DeriveInput);
//!
//!         // Append the fields.
//!         let fields_additional: FieldsUnnamed = parse_quote!((i64, usize));
//!         ast.append_unnamed(fields_additional);
//!
//!         // Example output:
//!         //
//!         // struct StructUnit(i64, usize);
//!         TokenStream::from(quote! { #ast })
//!     }
//!     ```
//!
//! 4. Get newtype inner `Field`.
//!
//!     This works for structs with unnamed fields or unit structs.
//!
//!     ```rust,ignore
//!     extern crate proc_macro;
//!
//!     use proc_macro::TokenStream;
//!     use proc_macro_roids::DeriveInputNewtypeExt;
//!     use quote::quote;
//!     use syn::{parse_macro_input, parse_quote, DeriveInput, Type};
//!
//!     #[proc_macro_derive(Deref)]
//!     pub fn derive_deref(item: TokenStream) -> TokenStream {
//!         // Example input:
//!         //
//!         // #[derive(Deref)]
//!         // struct Newtype(u32);
//!         let mut ast = parse_macro_input!(item as DeriveInput);
//!
//!         // Get the inner field.
//!         let inner_field = ast.inner_type();
//!
//!         // Implement `Deref`
//!         let type_name = &ast.ident;
//!         let token_stream_2 = quote! {
//!             #ast
//!
//!             impl std::ops::Deref for #type_name {
//!                 type Target = #inner_type;
//!                 fn deref(&self) -> &Self::Target {
//!                     &self.0
//!                 }
//!             }
//!         }
//!         TokenStream::from(token_stream_2)
//!     }
//!     ```
//!
//! 5. `Ident` concatenation.
//!
//!     ```rust,edition2018
//!     use proc_macro_roids::IdentExt;
//!     use proc_macro2::Span;
//!     use syn::Ident;
//!
//!     # fn main() {
//!     let one = Ident::new("One", Span::call_site());
//!     assert_eq!(Ident::new("OneSuffix", Span::call_site()), one.append("Suffix"));
//!     assert_eq!(Ident::new("PrefixOne", Span::call_site()), one.prepend("Prefix"));
//!
//!     let two = Ident::new("Two", Span::call_site());
//!     assert_eq!(Ident::new("OneTwo", Span::call_site()), one.append(&two));
//!     assert_eq!(Ident::new("TwoOne", Span::call_site()), one.prepend(&two));
//!     # }
//!     ```
//!
//! 6. Accessing struct fields.
//!
//!     ```rust,edition2018
//!     use proc_macro_roids::DeriveInputStructExt;
//!     use syn::{parse_quote, DeriveInput, Fields};
//!
//!     # fn main() {
//!     let ast: DeriveInput = parse_quote! {
//!         struct Named {}
//!     };
//!
//!     if let Fields::Named(..) = ast.fields() {
//!         // do something
//!     }
//!     # }
//!     ```
//!
//! 7. Inspecting `Field`s.
//!
//!     ```rust,edition2018
//!     use proc_macro_roids::FieldExt;
//!     use proc_macro2::Span;
//!     use syn::{parse_quote, Ident, Fields, FieldsNamed, Lit, LitStr, Meta, MetaNameValue};
//!
//!     # fn main() {
//!     let fields_named: FieldsNamed = parse_quote! {{
//!         #[my_derive(tag_name(param = "value"))]
//!         pub name: PhantomData<T>,
//!     }};
//!     let fields = Fields::from(fields_named);
//!     let field = fields.iter().next().expect("Expected field to exist.");
//!
//!     assert_eq!(field.type_name(), "PhantomData");
//!     assert!(field.is_phantom_data());
//!     assert!(field.contains_tag("my_derive", "tag_name"));
//!     assert_eq!(
//!         field.tag_parameter("my_derive", "tag_name").expect("Expected parameter to exist."),
//!         Meta::NameValue(MetaNameValue {
//!             ident: Ident::new("param", Span::call_site()),
//!             eq_token: Default::default(),
//!             lit: Lit::Str(LitStr::new("value", Span::call_site())),
//!         }),
//!     );
//!     # }
//!     ```

#[cfg(test)]
extern crate proc_macro;

pub use crate::{
    derive_input_ext::DeriveInputExt,
    derive_input_newtype_ext::DeriveInputNewtypeExt,
    derive_input_struct_ext::DeriveInputStructExt,
    field_ext::FieldExt,
    fields_named_append::FieldsNamedAppend,
    fields_unnamed_append::FieldsUnnamedAppend,
    ident_ext::IdentExt,
    util::{ident_concat, meta_list_contains, nested_meta_to_ident, tag_parameter, tag_parameters},
};

mod derive_input_ext;
mod derive_input_newtype_ext;
mod derive_input_struct_ext;
mod field_ext;
mod fields_named_append;
mod fields_unnamed_append;
mod ident_ext;
mod util;
