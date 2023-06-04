use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, Attribute, Ident, Meta, Path, Token};

/// Returns an `Ident` by concatenating `String` representations.
pub fn ident_concat(left: &str, right: &str) -> Ident {
    let mut combined = String::with_capacity(left.len() + right.len());
    combined.push_str(left);
    combined.push_str(right);

    Ident::new(&combined, Span::call_site())
}

/// Returns whether an item's attributes contains a given `#[namespace(tag)]`
/// attribute.
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
            let tags = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated);
            if let Ok(tags) = tags {
                tags.iter().any(|tag_existing| tag_existing.path() == tag)
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
/// ```rust,edition2021
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
/// let param_one = Meta::Path(meta_one);
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
pub fn namespace_parameter(attrs: &[Attribute], namespace: &Path) -> Option<Meta> {
    let mut namespace_nested_metas_iter = namespace_nested_metas_iter(attrs, namespace);
    let namespace_parameter = namespace_nested_metas_iter.next();
    let namespace_parameter_second = namespace_nested_metas_iter.next();

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
/// ```rust,edition2021
/// use proc_macro_roids::namespace_parameters;
/// use syn::{parse_quote, DeriveInput, Lit, LitStr, Meta, MetaNameValue, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(One, two = "")]
///     #[namespace(three(Value))]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let namespace_parameters = namespace_parameters(&ast.attrs, &ns);
///
/// let meta_one: Path = parse_quote!(One);
/// let param_one = Meta::Path(meta_one);
/// let meta_two: MetaNameValue = parse_quote!(two = "");
/// let param_two = Meta::NameValue(meta_two);
/// let meta_three: LitStr = parse_quote!("three");
/// let param_three = Meta::List(parse_quote!(three(Value)));
/// assert_eq!(
///     vec![param_one, param_two, param_three],
///     namespace_parameters
/// );
/// ```
pub fn namespace_parameters(attrs: &[Attribute], namespace: &Path) -> Vec<Meta> {
    let namespace_nested_metas_iter = namespace_nested_metas_iter(attrs, namespace);

    namespace_nested_metas_iter.collect::<Vec<Meta>>()
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
/// ```rust,edition2021
/// use proc_macro_roids::tag_parameter;
/// use syn::{parse_quote, DeriveInput, Meta, Path};
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
/// let param_one = Meta::Path(meta_one);
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
    let namespace_nested_metas_iter = namespace_nested_metas_iter(attrs, namespace);
    let mut tag_nested_metas_iter = tag_nested_metas_iter(namespace_nested_metas_iter, tag);
    let tag_param = tag_nested_metas_iter.next();
    let tag_param_second = tag_nested_metas_iter.next();

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
/// ```rust,edition2021
/// use proc_macro_roids::tag_parameters;
/// use syn::{parse_quote, DeriveInput, Meta, MetaNameValue, Path};
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
/// let param_one = Meta::Path(meta_one);
/// let meta_two: MetaNameValue = parse_quote!(two = "");
/// let param_two = Meta::NameValue(meta_two);
/// assert_eq!(vec![param_one, param_two], tag_parameters);
/// ```
pub fn tag_parameters(attrs: &[Attribute], namespace: &Path, tag: &Path) -> Vec<Meta> {
    let namespace_nested_metas_iter = namespace_nested_metas_iter(attrs, namespace);
    let parameters = tag_nested_metas_iter(namespace_nested_metas_iter, tag).collect::<Vec<Meta>>();

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
/// ```rust,edition2021
/// use proc_macro_roids::namespace_nested_metas_iter;
/// use syn::{parse_quote, DeriveInput, Meta, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(One)]
///     #[namespace(two = "")]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let nested_metas = namespace_nested_metas_iter(&ast.attrs, &ns).collect::<Vec<Meta>>();
///
/// let meta_one: Meta = Meta::Path(parse_quote!(One));
/// let meta_two: Meta = Meta::NameValue(parse_quote!(two = ""));
/// assert_eq!(vec![meta_one, meta_two], nested_metas);
/// ```
pub fn namespace_nested_metas_iter<'f>(
    attrs: &'f [Attribute],
    namespace: &'f Path,
) -> impl Iterator<Item = Meta> + 'f {
    attrs
        .iter()
        .filter_map(move |attr| {
            if attr.path() == namespace {
                attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .ok()
            } else {
                None
            }
        })
        .flat_map(|nested_metas| nested_metas.into_iter())
}

/// Returns the nested metas from within: `#[namespace(..)]`.
///
/// Each `meta` is a `namespace(..)` nested meta item.
///
/// # Parameters
///
/// * `attrs`: Attributes of the item to inspect.
/// * `namespace`: The `path()` of the first-level attribute.
///
/// # Examples
///
/// ```rust,edition2021
/// use proc_macro_roids::namespace_nested_metas;
/// use syn::{parse_quote, DeriveInput, Meta, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(One)]
///     #[namespace(two = "")]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let nested_metas = namespace_nested_metas(&ast.attrs, &ns);
///
/// let meta_one: Meta = Meta::Path(parse_quote!(One));
/// let meta_two: Meta = Meta::NameValue(parse_quote!(two = ""));
/// assert_eq!(vec![meta_one, meta_two], nested_metas);
pub fn namespace_nested_metas(attrs: &[Attribute], namespace: &Path) -> Vec<Meta> {
    namespace_nested_metas_iter(attrs, namespace).collect::<Vec<Meta>>()
}

/// Returns an iterator over nested metas from `#[namespace(tag(..))]`.
///
/// # Parameters
///
/// * `namespace_nested_metas_iter`: The `#[namespace(..)]` meta lists.
/// * `tag`: The `path()` of the second-level attribute.
///
/// # Examples
///
/// ```rust,edition2021
/// use proc_macro_roids::{namespace_nested_metas_iter, tag_nested_metas_iter};
/// use syn::{parse_quote, DeriveInput, Meta, Path};
///
/// let ast: DeriveInput = parse_quote! {
///     #[namespace(tag(One))]
///     #[namespace(tag(two = ""))]
///     pub struct MyEnum;
/// };
///
/// let ns: Path = parse_quote!(namespace);
/// let tag: Path = parse_quote!(tag);
/// let ns_lists = namespace_nested_metas_iter(&ast.attrs, &ns);
/// let nested_metas = tag_nested_metas_iter(ns_lists, &tag).collect::<Vec<Meta>>();
///
/// let meta_one: Meta = Meta::Path(parse_quote!(One));
/// let meta_two: Meta = Meta::NameValue(parse_quote!(two = ""));
/// assert_eq!(vec![meta_one, meta_two], nested_metas);
pub fn tag_nested_metas_iter<'f>(
    namespace_nested_metas_iter: impl Iterator<Item = Meta> + 'f,
    tag: &'f Path,
) -> impl Iterator<Item = Meta> + 'f {
    namespace_nested_metas_iter
        .filter_map(move |meta| {
            if meta.path() == tag {
                meta.require_list()
                    .and_then(|meta_list| {
                        meta_list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    })
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
