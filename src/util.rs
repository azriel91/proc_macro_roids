use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, Attribute, Ident, Meta, MetaList, Path, Token};

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
        .filter(|attr| attr.path() == namespace)
        .any(|attr| {
            let tags = attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated);
            if let Ok(tags) = tags {
                tags.iter().any(|tag_existing| tag_existing == tag)
            } else {
                false
            }
            // kcov-ignore-start
        })
    // kcov-ignore-end
}

/// Returns the parameter from `#[namespace(parameter)]`.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `path()` of the first-level attribute.
///
/// # Examples
///
/// ```rust
/// use proc_macro_roids::namespace_parameter;
/// use syn::{parse_quote, DeriveInput, Meta, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(One)]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let namespace_param = namespace_parameter(&ast.attrs, &ns);
///
/// let meta_one: Path = parse_quote!(One);
/// let param_one = NestedMeta::Meta(Meta::Path(meta_one));
/// assert_eq!(Some(param_one), namespace_param);
///
/// let ns_other: Path = parse_quote!(namespace_other);
/// let namespace_param_other = namespace_parameter(&ast.attrs, &ns_other);
/// assert_eq!(None, namespace_param_other);
/// ```
///
/// # Panics
///
/// Panics if the number of parameters for the tag is not exactly one.
#[allow(clippy::let_and_return)] // Needed due to bug in clippy.
pub fn namespace_parameter(attrs: &[Attribute], namespace: &Path) -> Option<MetaList> {
    let mut namespace_meta_lists_iter = namespace_meta_lists_iter(attrs, namespace);
    let namespace_parameter = namespace_meta_lists_iter.next();
    let namespace_parameter_second = namespace_meta_lists_iter.next();

    if namespace_parameter_second.is_some() {
        panic!(
            "Expected exactly one parameter for `#[{}(..)]`.",
            format_path(namespace),
        );
    }

    namespace_parameter
}

/// Returns the parameters from `#[namespace(param1, param2, ..)]`.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `path()` of the first-level attribute.
///
/// # Examples
///
/// ```rust
/// use proc_macro_roids::namespace_parameters;
/// use syn::{parse_quote, DeriveInput, Lit, LitStr, Meta, MetaNameValue, NestedMeta, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(One, two = "")]
///     #[namespace("three")]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let namespace_parameters = namespace_parameters(&ast.attrs, &ns);
///
/// let meta_one: Path = parse_quote!(One);
/// let param_one = NestedMeta::Meta(Meta::Path(meta_one));
/// let meta_two: MetaNameValue = parse_quote!(two = "");
/// let param_two = NestedMeta::Meta(Meta::NameValue(meta_two));
/// let meta_three: LitStr = parse_quote!("three");
/// let param_three = NestedMeta::Lit(Lit::Str(meta_three));
/// assert_eq!(
///     vec![param_one, param_two, param_three],
///     namespace_parameters
/// );
/// ```
pub fn namespace_parameters(attrs: &[Attribute], namespace: &Path) -> Vec<MetaList> {
    let namespace_meta_lists_iter = namespace_meta_lists_iter(attrs, namespace);
    let parameters = namespace_meta_lists_iter.collect::<Vec<MetaList>>();

    parameters
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
pub fn tag_parameter(attrs: &[Attribute], namespace: &Path, tag: &Path) -> Option<Meta> {
    let namespace_meta_lists_iter = namespace_meta_lists_iter(attrs, namespace);
    let mut tag_meta_lists_owned_iter = tag_meta_lists_owned_iter(namespace_meta_lists_iter, tag);
    let tag_param = tag_meta_lists_owned_iter.next();
    let tag_param_second = tag_meta_lists_owned_iter.next();

    if tag_param_second.is_some() {
        panic!(
            "Expected exactly one parameter for `#[{}({}(..))]`.",
            format_path(namespace),
            format_path(tag),
        );
    }

    tag_param
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
pub fn tag_parameters(attrs: &[Attribute], namespace: &Path, tag: &Path) -> Vec<Meta> {
    let namespace_meta_lists_iter = namespace_meta_lists_iter(attrs, namespace);
    let parameters =
        tag_meta_lists_owned_iter(namespace_meta_lists_iter, tag).collect::<Vec<Meta>>();

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
        .filter_map(move |attr| {
            if attr.path() == namespace {
                attr.parse_args_with(Punctuated::<MetaList, Token![,]>::parse_terminated)
                    .ok()
            } else {
                None
            }
        })
        .flat_map(|punctuated_meta_lists| punctuated_meta_lists.into_iter())
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
) -> impl Iterator<Item = MetaList> + 'f {
    namespace_meta_lists_iter
        .iter()
        .filter_map(|meta_list| {
            meta_list
                .parse_args_with(Punctuated::<MetaList, Token![,]>::parse_terminated)
                .ok()
        })
        .flatten()
        .filter(move |meta| &meta.path == tag)
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
) -> impl Iterator<Item = Meta> + 'f {
    namespace_meta_lists_iter
        .filter_map(move |meta_list| {
            if &meta_list.path == tag {
                meta_list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .ok()
            } else {
                None
            }
        })
        .flatten()
}

/// Returns a `Path` as a String without whitespace between tokens.
pub fn format_path(path: &Path) -> String {
    quote!(#path)
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
}
