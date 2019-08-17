use std::fmt::Display;

use proc_macro2::Span;
use syn::{Attribute, Ident, Meta, MetaList, NestedMeta};

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

/// Returns the parameter from `#[namespace(tag(parameter))]`.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `name()` of the first-level attribute.
/// * `tag`: The `name()` of the second-level attribute.
///
/// # Panics
///
/// Panics if the number of parameters for the tag is not exactly one.
#[allow(clippy::let_and_return)] // Needed due to bug in clippy.
pub fn tag_parameter<NS, Tag>(attrs: &[Attribute], namespace: NS, tag: Tag) -> Option<Meta>
where
    NS: Display,
    Tag: Display,
    Ident: PartialEq<NS>,
    Ident: PartialEq<Tag>,
{
    let error_message = format!(
        "Expected exactly one identifier for `#[{}({}(..))]`.",
        &namespace, &tag,
    );
    let namespace_meta_lists = namespace_meta_lists(attrs, namespace);
    let meta_param = tag_meta_list(&namespace_meta_lists, tag)
        // We want to insert a resource for each item in the list.
        .map(|meta_list| {
            if meta_list.nested.len() != 1 {
                panic!("{}. `{:?}`", &error_message, &meta_list.nested);
            }

            meta_list
                .nested
                .first()
                .map(|pair| {
                    let nested_meta = pair.value();
                    if let NestedMeta::Meta(meta) = nested_meta {
                        meta.clone()
                    } else {
                        panic!(
                            "`{:?}` is an invalid value in this position.\n\
                             Expected a single identifier.",
                            nested_meta,
                        );
                    }
                })
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
/// * `namespace`: The `name()` of the first-level attribute.
/// * `tag`: The `name()` of the second-level attribute.
pub fn tag_parameters<NS, Tag>(attrs: &[Attribute], namespace: NS, tag: Tag) -> Vec<Meta>
where
    Ident: PartialEq<NS>,
    Ident: PartialEq<Tag>,
{
    let namespace_meta_lists = namespace_meta_lists(attrs, namespace);
    let parameters = tag_meta_list(&namespace_meta_lists, tag)
        // We want to insert a resource for each item in the list.
        .flat_map(|meta_list| meta_list.nested.iter())
        .filter_map(|nested_meta| {
            if let NestedMeta::Meta(meta) = nested_meta {
                Some(meta.clone())
            } else {
                None
            }
        })
        .collect::<Vec<Meta>>();

    parameters
}

/// Returns the meta lists of the form: `#[namespace(..)]`.
///
/// Each `meta_list` is a `namespace(..)` meta item.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `name()` of the first-level attribute.
fn namespace_meta_lists<NS>(attrs: &[Attribute], namespace: NS) -> Vec<MetaList>
where
    Ident: PartialEq<NS>,
{
    attrs
        .iter()
        .map(Attribute::parse_meta)
        .filter_map(Result::ok)
        .filter(|meta| meta.name() == namespace)
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
/// # Parameters
///
/// * `namespace_meta_lists`: The `#[namespace(..)]` meta lists.
/// * `tag`: The `name()` of the second-level attribute.
fn tag_meta_list<'f, Tag>(
    namespace_meta_lists: &'f [MetaList],
    tag: Tag,
) -> impl Iterator<Item = &'f MetaList> + 'f
where
    Tag: 'f,
    Ident: PartialEq<Tag>,
{
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
        .filter(move |meta| meta.name() == tag)
        // `meta` is the `name(..)` item.
        .filter_map(|meta| {
            if let Meta::List(meta_list) = meta {
                Some(meta_list)
            } else {
                None
            }
        })
}
