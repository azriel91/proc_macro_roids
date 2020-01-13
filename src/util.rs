use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Pair, Attribute, Ident, Meta, MetaList, NestedMeta, Path};

/// Returns the `Path` of a nested meta. If it is a literal, `None` is returned.
///
/// # Parameters
///
/// * `nested_meta`: The `NestedMeta` to extract the `Path` from.
pub fn nested_meta_to_path(nested_meta: &NestedMeta) -> Option<&Path> {
    // kcov-ignore-start
    match nested_meta {
        // kcov-ignore-end
        NestedMeta::Meta(meta) => Some(meta.path()),
        NestedMeta::Lit(..) => None, // kcov-ignore
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
                            None // kcov-ignore
                        }
                    })
                    .any(|meta| meta.path() == tag)
            } else {
                false
            }
            // kcov-ignore-start
        })
    // kcov-ignore-end
}

/// Returns the parameter from `#[namespace(tag(parameter))]`.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `path()` of the first-level attribute.
/// * `tag`: The `path()` of the second-level attribute.
///
/// # Examples
///
/// ```rust
/// use proc_macro_roids::tag_parameter;
/// use syn::{parse_quote, DeriveInput, Meta, NestedMeta, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(tag(One))]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let tag: Path = parse_quote!(tag);
/// let tag_param = tag_parameter(&ast.attrs, &ns, &tag);
///
/// let meta_one: Path = parse_quote!(One);
/// let param_one = NestedMeta::Meta(Meta::Path(meta_one));
/// assert_eq!(Some(param_one), tag_param);
///
/// let tag_other: Path = parse_quote!(tag_other);
/// let tag_param_other = tag_parameter(&ast.attrs, &ns, &tag_other);
/// assert_eq!(None, tag_param_other);
/// ```
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
    let namespace_meta_lists_iter = namespace_meta_lists_iter(attrs, namespace);
    let meta_param = tag_meta_lists_owned_iter(namespace_meta_lists_iter, tag)
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
///
/// # Examples
///
/// ```rust
/// use proc_macro_roids::tag_parameters;
/// use syn::{parse_quote, DeriveInput, Meta, MetaNameValue, NestedMeta, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(tag(One))]
///     #[namespace(tag(two = ""))]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let tag: Path = parse_quote!(tag);
/// let tag_parameters = tag_parameters(&ast.attrs, &ns, &tag);
///
/// let meta_one: Path = parse_quote!(One);
/// let param_one = NestedMeta::Meta(Meta::Path(meta_one));
/// let meta_two: MetaNameValue = parse_quote!(two = "");
/// let param_two = NestedMeta::Meta(Meta::NameValue(meta_two));
/// assert_eq!(vec![param_one, param_two], tag_parameters);
/// ```
pub fn tag_parameters(attrs: &[Attribute], namespace: &Path, tag: &Path) -> Vec<NestedMeta> {
    let namespace_meta_lists_iter = namespace_meta_lists_iter(attrs, namespace);
    let parameters = tag_meta_lists_owned_iter(namespace_meta_lists_iter, tag)
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
///
/// # Examples
///
/// ```rust
/// use proc_macro_roids::namespace_meta_lists_iter;
/// use syn::{parse_quote, DeriveInput, MetaList, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(One)]
///     #[namespace(two = "")]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let lists = namespace_meta_lists_iter(&ast.attrs, &ns).collect::<Vec<MetaList>>();
///
/// let list_one: MetaList = parse_quote!(namespace(One));
/// let list_two: MetaList = parse_quote!(namespace(two = ""));
/// assert_eq!(vec![list_one, list_two], lists);
/// ```
pub fn namespace_meta_lists_iter<'f>(
    attrs: &'f [Attribute],
    namespace: &'f Path,
) -> impl Iterator<Item = MetaList> + 'f {
    attrs
        .iter()
        .map(Attribute::parse_meta)
        .filter_map(Result::ok)
        .filter(move |meta| meta.path() == namespace)
        .filter_map(|meta| {
            if let Meta::List(meta_list) = meta {
                Some(meta_list)
            } else {
                None
            }
        })
}

/// Returns the meta lists of the form: `#[namespace(..)]`.
///
/// Each `meta_list` is a `namespace(..)` meta item.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `path()` of the first-level attribute.
///
/// # Examples
///
/// ```rust
/// use proc_macro_roids::namespace_meta_lists;
/// use syn::{parse_quote, DeriveInput, MetaList, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(One)]
///     #[namespace(two = "")]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let lists = namespace_meta_lists(&ast.attrs, &ns);
///
/// let list_one: MetaList = parse_quote!(namespace(One));
/// let list_two: MetaList = parse_quote!(namespace(two = ""));
/// assert_eq!(vec![list_one, list_two], lists);
pub fn namespace_meta_lists(attrs: &[Attribute], namespace: &Path) -> Vec<MetaList> {
    namespace_meta_lists_iter(attrs, namespace).collect::<Vec<MetaList>>()
}

/// Returns an iterator over meta lists from `#[namespace(tag(..))]`.
///
/// For an owned version of the iterator, see `tag_meta_lists_owned_iter`
///
/// # Parameters
///
/// * `namespace_meta_lists_iter`: The `#[namespace(..)]` meta lists.
/// * `tag`: The `path()` of the second-level attribute.
///
/// # Examples
///
/// ```rust
/// use proc_macro_roids::{namespace_meta_lists, tag_meta_lists_iter};
/// use syn::{parse_quote, DeriveInput, MetaList, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(tag(One))]
///     #[namespace(tag(two = ""))]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let tag: Path = parse_quote!(tag);
/// let ns_lists = namespace_meta_lists(&ast.attrs, &ns);
/// let lists = tag_meta_lists_iter(&ns_lists, &tag).collect::<Vec<&MetaList>>();
///
/// let list_one: MetaList = parse_quote!(tag(One));
/// let list_two: MetaList = parse_quote!(tag(two = ""));
/// assert_eq!(vec![&list_one, &list_two], lists);
pub fn tag_meta_lists_iter<'f>(
    namespace_meta_lists_iter: &'f [MetaList],
    tag: &'f Path,
) -> impl Iterator<Item = &'f MetaList> + 'f {
    namespace_meta_lists_iter
        .iter()
        .flat_map(|meta_list| meta_list.nested.iter())
        .filter_map(|nested_meta| {
            if let NestedMeta::Meta(meta) = nested_meta {
                Some(meta)
            } else {
                None // kcov-ignore
            }
        })
        .filter(move |meta| meta.path() == tag)
        // `meta` is the `tag(..)` item.
        .filter_map(|meta| {
            if let Meta::List(meta_list) = meta {
                Some(meta_list)
            } else {
                None // kcov-ignore
            }
        })
}

/// Returns an iterator over meta lists from `#[namespace(tag(..))]`.
///
/// # Parameters
///
/// * `namespace_meta_lists_iter`: The `#[namespace(..)]` meta lists.
/// * `tag`: The `path()` of the second-level attribute.
///
/// # Examples
///
/// ```rust
/// use proc_macro_roids::{namespace_meta_lists_iter, tag_meta_lists_owned_iter};
/// use syn::{parse_quote, DeriveInput, MetaList, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(tag(One))]
///     #[namespace(tag(two = ""))]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let tag: Path = parse_quote!(tag);
/// let ns_lists_iter = namespace_meta_lists_iter(&ast.attrs, &ns);
/// let lists = tag_meta_lists_owned_iter(ns_lists_iter, &tag).collect::<Vec<MetaList>>();
///
/// let list_one: MetaList = parse_quote!(tag(One));
/// let list_two: MetaList = parse_quote!(tag(two = ""));
/// assert_eq!(vec![list_one, list_two], lists);
pub fn tag_meta_lists_owned_iter<'f>(
    namespace_meta_lists_iter: impl Iterator<Item = MetaList> + 'f,
    tag: &'f Path,
) -> impl Iterator<Item = MetaList> + 'f {
    namespace_meta_lists_iter
        .flat_map(|meta_list| meta_list.nested.into_pairs().map(Pair::into_value))
        .filter_map(|nested_meta| {
            if let NestedMeta::Meta(meta) = nested_meta {
                Some(meta)
            } else {
                None // kcov-ignore
            }
        })
        .filter(move |meta| meta.path() == tag)
        // `meta` is the `tag(..)` item.
        .filter_map(|meta| {
            if let Meta::List(meta_list) = meta {
                Some(meta_list)
            } else {
                None // kcov-ignore
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
