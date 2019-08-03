use syn::{
    punctuated::Pair, Attribute, Field, Ident, Meta, NestedMeta, PathSegment, Type, TypePath,
};

/// Functions to make it ergonomic to inspect `Field`s and their attributes.
pub trait FieldExt {
    /// Returns whether a field contains a given `#[namespace(tag)]` attribute.
    ///
    /// # Parameters
    ///
    /// * `namespace`: The `name()` of the first-level attribute.
    /// * `tag`: The `name()` of the second-level attribute.
    fn contains_tag<NS, Tag>(&self, namespace: NS, tag: Tag) -> bool
    where
        Ident: PartialEq<NS>,
        Ident: PartialEq<Tag>;

    /// Returns the simple type name of a field.
    ///
    /// For example, the `PhantomData` in `std::marker::PhantomData<T>`.
    fn type_name(&self) -> &Ident;

    /// Returns whether the field is `PhantomData`.
    ///
    /// Note that the detection is a string comparison instead of a type ID comparison, so is prone
    /// to inaccurate detection, for example:
    ///
    /// * `use std::marker::PhantomData as GhostData;`
    /// * `use other_crate::OtherType as PhantomData;`
    fn is_phantom_data(&self) -> bool;
}

impl FieldExt for Field {
    fn contains_tag<NS, Tag>(&self, namespace: NS, tag: Tag) -> bool
    where
        Ident: PartialEq<NS>,
        Ident: PartialEq<Tag>,
    {
        self.attrs
            .iter()
            .map(Attribute::parse_meta)
            .filter_map(Result::ok)
            .filter(|meta| meta.name() == namespace)
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
                        .any(|meta| meta.name() == tag)
                } else {
                    false
                }
            })
    }

    fn type_name(&self) -> &Ident {
        if let Type::Path(TypePath { path, .. }) = &self.ty {
            if let Some(Pair::End(PathSegment { ident, .. })) = path.segments.last() {
                return ident;
            }
        }
        panic!(
            "Expected {}field type to be a `Path` with a segment.",
            self.ident
                .as_ref()
                .map(|ident| format!("`{:?}` ", ident))
                .unwrap_or_else(|| String::from(""))
        );
    }

    fn is_phantom_data(&self) -> bool {
        self.type_name() == "PhantomData"
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, Fields, FieldsNamed};

    use super::FieldExt;

    #[test]
    fn type_name_returns_simple_type_name() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my_derive(tag_name)]
            pub name: PhantomData<T>,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert_eq!(field.type_name(), "PhantomData");
    }

    #[test]
    fn is_phantom_data_returns_true_for_phantom_data() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my_derive(tag_name)]
            pub name: PhantomData<T>,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert!(field.is_phantom_data());
    }

    #[test]
    fn is_phantom_data_returns_false_for_non_phantom_data() {
        let fields_named: FieldsNamed = parse_quote! {{
            #[my_derive(tag_name)]
            pub name: GhostData<T>,
        }};
        let fields = Fields::from(fields_named);
        let field = fields.iter().next().expect("Expected field to exist.");

        assert!(!field.is_phantom_data());
    }

    mod fields_named {
        use proc_macro2::Span;
        use quote::quote;
        use syn::{parse_quote, Error, Fields, FieldsNamed};

        use super::super::FieldExt;

        #[test]
        fn contains_tag_returns_true_when_tag_exists() -> Result<(), Error> {
            let fields_named: FieldsNamed = parse_quote! {{
                #[my_derive(tag_name)]
                pub name: PhantomData,
            }};
            let fields = Fields::from(fields_named);
            let field = fields.iter().next().expect("Expected field to exist.");

            assert!(field.contains_tag("my_derive", "tag_name"));

            Ok(())
        }

        #[test]
        fn contains_tag_returns_false_when_tag_does_not_exist() -> Result<(), Error> {
            let tokens_list = vec![
                quote! {{
                    #[my_derive]
                    pub name: PhantomData,
                }},
                quote! {{
                    #[my_derive(other)]
                    pub name: PhantomData,
                }},
                quote! {{
                    #[other(tag_name)]
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

                    assert!(
                        !field.contains_tag("my_derive", "tag_name"),
                        assertion_message
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
                #[my_derive(tag_name)]
                pub PhantomData,
            )};
            let fields = Fields::from(fields_unnamed);
            let field = fields.iter().next().expect("Expected field to exist.");

            assert!(field.contains_tag("my_derive", "tag_name"));

            Ok(())
        }

        #[test]
        fn contains_tag_returns_false_when_tag_does_not_exist() -> Result<(), Error> {
            let tokens_list = vec![
                quote! {(
                    #[my_derive]
                    pub PhantomData,
                )},
                quote! {(
                    #[my_derive(other)]
                    pub PhantomData,
                )},
                quote! {(
                    #[other(tag_name)]
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

                    assert!(
                        !field.contains_tag("my_derive", "tag_name"),
                        assertion_message
                    );

                    Ok(())
                })
        }
    }
}
