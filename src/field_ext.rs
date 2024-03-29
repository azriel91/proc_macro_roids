use syn::{Field, Ident, Meta, Path, PathSegment, Type, TypePath};

use crate::util;

/// Functions to make it ergonomic to inspect `Field`s and their attributes.
pub trait FieldExt {
    /// Returns the simple type name of a field.
    ///
    /// For example, the `PhantomData` in `std::marker::PhantomData<T>`.
    fn type_name(&self) -> &Ident;

    /// Returns whether the field is `PhantomData`.
    ///
    /// Note that the detection is a string comparison instead of a type ID
    /// comparison, so is prone to inaccurate detection, for example:
    ///
    /// * `use std::marker::PhantomData as GhostData;`
    /// * `use other_crate::OtherType as PhantomData;`
    fn is_phantom_data(&self) -> bool;

    /// Returns whether a field contains a given `#[namespace(tag)]` attribute.
    ///
    /// # Parameters
    ///
    /// * `namespace`: The `path()` of the first-level attribute.
    /// * `tag`: The `path()` of the second-level attribute.
    fn contains_tag(&self, namespace: &Path, tag: &Path) -> bool;

    /// Returns the parameter from `#[namespace(parameter)]`.
    ///
    /// # Parameters
    ///
    /// * `namespace`: The `path()` of the first-level attribute.
    ///
    /// # Panics
    ///
    /// Panics if there is more than one parameter for the tag.
    fn namespace_parameter(&self, namespace: &Path) -> Option<Meta>;

    /// Returns the parameters from `#[namespace(param1, param2, ..)]`.
    ///
    /// # Parameters
    ///
    /// * `namespace`: The `path()` of the first-level attribute.
    fn namespace_parameters(&self, namespace: &Path) -> Vec<Meta>;

    /// Returns the parameter from `#[namespace(tag(parameter))]`.
    ///
    /// # Parameters
    ///
    /// * `namespace`: The `path()` of the first-level attribute.
    /// * `tag`: The `path()` of the second-level attribute.
    ///
    /// # Panics
    ///
    /// Panics if there is more than one parameter for the tag.
    fn tag_parameter(&self, namespace: &Path, tag: &Path) -> Option<Meta>;

    /// Returns the parameters from `#[namespace(tag(param1, param2, ..))]`.
    ///
    /// # Parameters
    ///
    /// * `namespace`: The `path()` of the first-level attribute.
    /// * `tag`: The `path()` of the second-level attribute.
    fn tag_parameters(&self, namespace: &Path, tag: &Path) -> Vec<Meta>;
}

impl FieldExt for Field {
    fn type_name(&self) -> &Ident {
        if let Type::Path(TypePath { path, .. }) = &self.ty {
            if let Some(PathSegment { ident, .. }) = path.segments.last() {
                return ident;
            }
        }
        // kcov-ignore-start
        panic!(
            "Expected {}field type to be a `Path` with a segment.",
            self.ident
                .as_ref()
                .map(|ident| format!("`{:?}` ", ident))
                .unwrap_or_else(|| String::from(""))
        );
        // kcov-ignore-end
    }

    fn is_phantom_data(&self) -> bool {
        self.type_name() == "PhantomData"
    }

    fn contains_tag(&self, namespace: &Path, tag: &Path) -> bool {
        util::contains_tag(&self.attrs, namespace, tag)
    }

    fn namespace_parameter(&self, namespace: &Path) -> Option<Meta> {
        util::namespace_parameter(&self.attrs, namespace)
    }

    fn namespace_parameters(&self, namespace: &Path) -> Vec<Meta> {
        util::namespace_parameters(&self.attrs, namespace)
    }

    fn tag_parameter(&self, namespace: &Path, tag: &Path) -> Option<Meta> {
        util::tag_parameter(&self.attrs, namespace, tag)
    }

    fn tag_parameters(&self, namespace: &Path, tag: &Path) -> Vec<Meta> {
        util::tag_parameters(&self.attrs, namespace, tag)
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, Fields, FieldsNamed, Meta, MetaNameValue};

    use super::FieldExt;

    #[test]
    fn type_name_returns_simple_type_name() {
        let fields_named: FieldsNamed = parse_quote! {{
            pub name: PhantomData<T>,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert_eq!(field.type_name(), "PhantomData");
    }

    #[test]
    fn is_phantom_data_returns_true_for_phantom_data() {
        let fields_named: FieldsNamed = parse_quote! {{
            pub name: PhantomData<T>,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert!(field.is_phantom_data());
    }

    #[test]
    fn is_phantom_data_returns_false_for_non_phantom_data() {
        let fields_named: FieldsNamed = parse_quote! {{
            pub name: GhostData<T>,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert!(!field.is_phantom_data());
    }

    #[test]
    fn namespace_parameter_returns_none_when_not_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[other::derive]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        let parameter = field.namespace_parameter(&parse_quote!(my::derive));
        assert_eq!(parameter, None);
    }

    #[test]
    fn namespace_parameter_returns_meta_when_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive(Magic)]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert_eq!(
            field.namespace_parameter(&parse_quote!(my::derive)),
            Some(Meta::Path(parse_quote!(Magic)))
        );
    }

    #[test]
    #[should_panic(expected = "Expected exactly one parameter for `#[my::derive(..)]`.")]
    fn namespace_parameter_panics_when_multiple_parameters_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive(Magic::One, Magic::Two)]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        field.namespace_parameter(&parse_quote!(my::derive));
    }

    #[test]
    fn namespace_parameters_returns_empty_vec_when_not_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert_eq!(
            field.namespace_parameters(&parse_quote!(my::derive)),
            Vec::<Meta>::new()
        );
    }

    #[test]
    fn namespace_parameters_returns_metas_when_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive(Magic::One, second = "{ Magic::Two }")]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert_eq!(
            field.namespace_parameters(&parse_quote!(my::derive)),
            vec![
                Meta::Path(parse_quote!(Magic::One)),
                Meta::NameValue(MetaNameValue {
                    path: parse_quote!(second),
                    eq_token: Default::default(),
                    value: parse_quote!("{ Magic::Two }")
                }),
            ]
        );
    }

    #[test]
    fn tag_parameter_returns_none_when_not_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        let parameter = field.tag_parameter(&parse_quote!(my::derive), &parse_quote!(tag::name));
        assert_eq!(parameter, None);
    }

    #[test]
    fn tag_parameter_returns_meta_when_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive(tag::name(Magic))]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert_eq!(
            field.tag_parameter(&parse_quote!(my::derive), &parse_quote!(tag::name)),
            Some(Meta::Path(parse_quote!(Magic)))
        );
    }

    #[test]
    #[should_panic(expected = "Expected exactly one parameter for `#[my::derive(tag::name(..))]`.")]
    fn tag_parameter_panics_when_multiple_parameters_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive(tag::name(Magic::One, Magic::Two))]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        field.tag_parameter(&parse_quote!(my::derive), &parse_quote!(tag::name));
    }

    #[test]
    fn tag_parameters_returns_empty_vec_when_not_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert_eq!(
            field.tag_parameters(&parse_quote!(my::derive), &parse_quote!(tag::name)),
            Vec::<Meta>::new()
        );
    }

    #[test]
    fn tag_parameters_returns_metas_when_present() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my::derive(tag::name(Magic::One, second = "{ Magic::Two }"))]
            pub name: u32,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert_eq!(
            field.tag_parameters(&parse_quote!(my::derive), &parse_quote!(tag::name)),
            vec![
                Meta::Path(parse_quote!(Magic::One)),
                Meta::NameValue(MetaNameValue {
                    path: parse_quote!(second),
                    eq_token: Default::default(),
                    value: parse_quote!("{ Magic::Two }")
                }),
            ]
        );
    }

    mod fields_named {
        use proc_macro2::Span;
        use quote::quote;
        use syn::{parse_quote, Error, Fields, FieldsNamed};

        use super::super::FieldExt;

        #[test]
        fn contains_tag_returns_true_when_tag_exists() -> Result<(), Error> {
            let fields_named: FieldsNamed = parse_quote! {{
                #[my::derive(tag::name)]
                pub name: PhantomData,
            }};
            let fields = Fields::from(fields_named);
            let field = fields.iter().next().expect("Expected field to exist.");

            assert!(field.contains_tag(&parse_quote!(my::derive), &parse_quote!(tag::name)));

            Ok(())
        }

        #[test]
        fn contains_tag_returns_false_when_tag_does_not_exist() -> Result<(), Error> {
            let tokens_list = vec![
                quote! {{
                    #[my::derive]
                    pub name: PhantomData,
                }},
                quote! {{
                    #[my::derive(other)]
                    pub name: PhantomData,
                }},
                quote! {{
                    #[other(tag::name)]
                    pub name: PhantomData,
                }},
            ];

            tokens_list
                .into_iter()
                .try_for_each(|tokens| -> Result<(), Error> {
                    let message = format!("Failed to parse tokens: `{}`", &tokens);
                    let assertion_message = format!(
                        "Expected `contains_tag` to return false for tokens: `{}`",
                        &tokens
                    );

                    let fields_named: FieldsNamed =
                        syn::parse2(tokens).map_err(|_| Error::new(Span::call_site(), &message))?;
                    let fields = Fields::from(fields_named);
                    let field = fields.iter().next().expect("Expected field to exist.");

                    // kcov-ignore-start
                    assert!(
                        // kcov-ignore-end
                        !field.contains_tag(&parse_quote!(my::derive), &parse_quote!(tag::name)),
                        "{}",              // kcov-ignore
                        assertion_message  // kcov-ignore
                    );

                    Ok(())
                })
        }
    }

    mod fields_unnamed {
        use proc_macro2::Span;
        use quote::quote;
        use syn::{parse_quote, Error, Fields, FieldsUnnamed};

        use super::super::FieldExt;

        #[test]
        fn contains_tag_returns_true_when_tag_exists() -> Result<(), Error> {
            let fields_unnamed: FieldsUnnamed = parse_quote! {(
                #[my::derive(tag::name)]
                pub PhantomData,
            )};
            let fields = Fields::from(fields_unnamed);
            let field = fields.iter().next().expect("Expected field to exist.");

            assert!(field.contains_tag(&parse_quote!(my::derive), &parse_quote!(tag::name)));

            Ok(())
        }

        #[test]
        fn contains_tag_returns_false_when_tag_does_not_exist() -> Result<(), Error> {
            let tokens_list = vec![
                quote! {(
                    #[my::derive]
                    pub PhantomData,
                )},
                quote! {(
                    #[my::derive(other)]
                    pub PhantomData,
                )},
                quote! {(
                    #[other(tag::name)]
                    pub PhantomData,
                )},
            ];

            tokens_list
                .into_iter()
                .try_for_each(|tokens| -> Result<(), Error> {
                    let message = format!("Failed to parse tokens: `{}`", &tokens);
                    let assertion_message = format!(
                        "Expected `contains_tag` to return false for tokens: `{}`",
                        &tokens
                    );

                    let fields_unnamed: FieldsUnnamed =
                        syn::parse2(tokens).map_err(|_| Error::new(Span::call_site(), &message))?;
                    let fields = Fields::from(fields_unnamed);
                    let field = fields.iter().next().expect("Expected field to exist.");

                    // kcov-ignore-start
                    assert!(
                        // kcov-ignore-end
                        !field.contains_tag(&parse_quote!(my::derive), &parse_quote!(tag::name)),
                        "{}",              // kcov-ignore
                        assertion_message  // kcov-ignore
                    );

                    Ok(())
                })
        }
    }
}
