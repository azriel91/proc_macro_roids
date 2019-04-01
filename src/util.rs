use proc_macro2::Span;
use syn::{Ident, MetaList, NestedMeta};

/// Returns the `Ident` of a nested meta. If it is a literal, `None` is returned.
///
/// # Parameters
///
/// * `nested_meta`: The `NestedMeta` to extract the `Ident` from.
pub fn nested_meta_to_ident(nested_meta: &NestedMeta) -> Option<Ident> {
    match nested_meta {
        NestedMeta::Meta(meta) => Some(meta.name()),
        NestedMeta::Literal(..) => None,
    }
}

/// Returns whether the `MetaList` contains a `Meta::Word` with the given ident.
///
/// This can be used to check if a `#[derive(..)]` contains `SomeDerive`.
///
/// # Parameters
///
/// * `meta_list`: The `MetaList` to check.
/// * `operand`: `NestedMeta` that may be in the list.
pub fn meta_list_contains(meta_list: &MetaList, operand: &NestedMeta) -> bool {
    meta_list
        .nested
        .iter()
        .any(|nested_meta| nested_meta == operand)
}

/// Returns an `Ident` by concatenating `String` representations.
pub fn ident_concat(left: &str, right: &str) -> Ident {
    let mut combined = String::with_capacity(left.len() + right.len());
    combined.push_str(left);
    combined.push_str(right);

    Ident::new(&combined, Span::call_site())
}
