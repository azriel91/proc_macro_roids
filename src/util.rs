use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Pair, Attribute, Ident, Meta, MetaList, NestedMeta, Path};

/// Returns the `Path` of a nested meta. If it is a literal, `None` is returned.
///
/// # Parameters
///
/// * `nested_meta`: The `NestedMeta` to extract the `Path` from.
pub fn nested_meta_to_path(nested_meta: &NestedMeta) -> Option<&Path> {
    match nested_meta {
        NestedMeta::Meta(meta) => Some(meta.path()),
        NestedMeta::Lit(..) => None,
    }
}

/// Returns whether the `MetaList` contains the specified `NestedMeta`.
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

/// Returns whether an item's attributes contains a given `#[namespace(tag)]` attribute.
///
/// # Parameters
///
/// * `attrs`: The attributes on the item.
/// * `namespace`: The `path()` of the first-level attribute.
/// * `tag`: The `path()` of the second-level attribute.
pub fn contains_tag(attrs: &[Attribute], namespace: &Path, tag: &Path) -> bool {
    attrs
        .iter()
        .map(Attribute::parse_meta)
        .filter_map(Result::ok)
        .filter(|meta| meta.path() == namespace)
        .any(|meta| {
            if let Meta::List(meta_list) = meta {
                meta_list
                    .nested
                    .iter()
                    .filter_map(|nested_meta| {
                        if let NestedMeta::Meta(meta) = nested_meta {
                            Some(meta)
                        } else {
                            None
                        }
                    })
                    .any(|meta| meta.path() == tag)
            } else {
                false
            }
        })
}

/// Returns the parameter from `#[namespace(tag(parameter))]`.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `path()` of the first-level attribute.
/// * `tag`: The `path()` of the second-level attribute.
///
/// # Panics
///
/// Panics if the number of parameters for the tag is not exactly one.
#[allow(clippy::let_and_return)] // Needed due to bug in clippy.
pub fn tag_parameter(attrs: &[Attribute], namespace: &Path, tag: &Path) -> Option<NestedMeta> {
    let error_message = {
        format!(
            "Expected exactly one identifier for `#[{}({}(..))]`.",
            format_path(namespace),
            format_path(tag),
        )
    };
    let namespace_meta_lists = namespace_meta_lists(attrs, namespace);
    let meta_param = tag_meta_list_owned(namespace_meta_lists, tag)
        // We want to insert a resource for each item in the list.
        .map(|meta_list| {
            if meta_list.nested.len() != 1 {
                panic!("{}. `{:?}`", &error_message, &meta_list.nested);
            }

            meta_list
                .nested
                .into_pairs()
                .map(Pair::into_value)
                .next()
                .expect("Expected one meta item to exist.")
        })
        .next();

    meta_param
}

/// Returns the parameters from `#[namespace(tag(param1, param2, ..))]`.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `path()` of the first-level attribute.
/// * `tag`: The `path()` of the second-level attribute.
pub fn tag_parameters(attrs: &[Attribute], namespace: &Path, tag: &Path) -> Vec<NestedMeta> {
    let namespace_meta_lists = namespace_meta_lists(attrs, namespace);
    let parameters = tag_meta_list_owned(namespace_meta_lists, tag)
        .flat_map(|meta_list| meta_list.nested.into_pairs().map(Pair::into_value))
        .collect::<Vec<NestedMeta>>();

    parameters
}

/// Returns the meta lists of the form: `#[namespace(..)]`.
///
/// Each `meta_list` is a `namespace(..)` meta item.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `path()` of the first-level attribute.
pub fn namespace_meta_lists(attrs: &[Attribute], namespace: &Path) -> Vec<MetaList> {
    attrs
        .iter()
        .map(Attribute::parse_meta)
        .filter_map(Result::ok)
        .filter(|meta| meta.path() == namespace)
        .filter_map(|meta| {
            if let Meta::List(meta_list) = meta {
                Some(meta_list)
            } else {
                None
            }
        })
        .collect::<Vec<MetaList>>()
}

/// Returns an iterator over meta lists from `#[namespace(tag(..))]`.
///
/// For an owned version of the iterator, see `tag_meta_list_owned`
///
/// # Parameters
///
/// * `namespace_meta_lists`: The `#[namespace(..)]` meta lists.
/// * `tag`: The `path()` of the second-level attribute.
pub fn tag_meta_list<'f>(
    namespace_meta_lists: &'f [MetaList],
    tag: &'f Path,
) -> impl Iterator<Item = &'f MetaList> + 'f {
    namespace_meta_lists
        .iter()
        .flat_map(|meta_list| meta_list.nested.iter())
        .filter_map(|nested_meta| {
            if let NestedMeta::Meta(meta) = nested_meta {
                Some(meta)
            } else {
                None
            }
        })
        .filter(move |meta| meta.path() == tag)
        // `meta` is the `tag(..)` item.
        .filter_map(|meta| {
            if let Meta::List(meta_list) = meta {
                Some(meta_list)
            } else {
                None
            }
        })
}

/// Returns an iterator over meta lists from `#[namespace(tag(..))]`.
///
/// # Parameters
///
/// * `namespace_meta_lists`: The `#[namespace(..)]` meta lists.
/// * `tag`: The `path()` of the second-level attribute.
pub fn tag_meta_list_owned<'f>(
    namespace_meta_lists: Vec<MetaList>,
    tag: &'f Path,
) -> impl Iterator<Item = MetaList> + 'f {
    namespace_meta_lists
        .into_iter()
        .flat_map(|meta_list| meta_list.nested.into_pairs().map(Pair::into_value))
        .filter_map(|nested_meta| {
            if let NestedMeta::Meta(meta) = nested_meta {
                Some(meta)
            } else {
                None
            }
        })
        .filter(move |meta| meta.path() == tag)
        // `meta` is the `tag(..)` item.
        .filter_map(|meta| {
            if let Meta::List(meta_list) = meta {
                Some(meta_list)
            } else {
                None
            }
        })
}

/// Returns a `Path` as a String without whitespace between tokens.
pub fn format_path(path: &Path) -> String {
    quote!(#path)
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
}
