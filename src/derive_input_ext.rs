use syn::{
    parse_quote, punctuated::Punctuated, Attribute, DeriveInput, Meta, NestedMeta, Path, Token,
};

use crate::{meta_list_contains, nested_meta_to_path, util};

/// Functions to make it ergonomic to work with `struct` ASTs.
pub trait DeriveInputExt {
    /// Appends derives to the list of derives.
    ///
    /// **Note:** This can only be used with [*attribute*] macros, and not [*derive*] macros.
    ///
    /// * If the `derive` attribute does not exist, one will be created.
    /// * If the `derive` attribute exists, and there are existing `derive`s that overlap with the
    ///   derives to append, this macro will panic with the overlapping derives.
    /// * If the `derive` attribute exists, and there are no overlapping `derive`s, then they will
    ///   be combined.
    ///
    /// # Panics
    ///
    /// Panics if there are existing `derive`s that overlap with the derives to append.
    ///
    /// [*attribute*]: <https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros>
    /// [*derive*]: <https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros>
    fn append_derives(&mut self, derives: Punctuated<NestedMeta, Token![,]>);

    /// Returns whether the type contains a given `#[namespace(tag)]` attribute.
    ///
    /// # Parameters
    ///
    /// * `namespace`: The `path()` of the first-level attribute.
    /// * `tag`: The `path()` of the second-level attribute.
    fn contains_tag(&self, namespace: &Path, tag: &Path) -> bool;

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
    fn tag_parameter(&self, namespace: &Path, tag: &Path) -> Option<NestedMeta>;

    /// Returns the parameters from `#[namespace(tag(param1, param2, ..))]`.
    ///
    /// # Parameters
    ///
    /// * `namespace`: The `path()` of the first-level attribute.
    /// * `tag`: The `path()` of the second-level attribute.
    fn tag_parameters(&self, namespace: &Path, tag: &Path) -> Vec<NestedMeta>;
}

impl DeriveInputExt for DeriveInput {
    fn append_derives(&mut self, derives_to_append: Punctuated<NestedMeta, Token![,]>) {
        let attr_derives_existing = self
            .attrs
            .iter_mut()
            .filter(|attr| attr.path.is_ident("derive"))
            .filter_map(|attr| match attr.parse_meta() {
                Ok(Meta::List(meta_list)) => Some((attr, meta_list)),
                _ => None,
            })
            .next();

        if let Some((attr, mut derives_existing)) = attr_derives_existing {
            // Emit warning if the user derives any of the existing derives, as we do that for them.
            let superfluous = derives_to_append
                .iter()
                .filter(|derive_to_append| meta_list_contains(&derives_existing, derive_to_append))
                .filter_map(nested_meta_to_path)
                .map(util::format_path)
                .collect::<Vec<_>>();
            if !superfluous.is_empty() {
                // TODO: Emit warning, pending <https://github.com/rust-lang/rust/issues/54140>
                // derives_existing
                //     .span()
                //     .warning(
                //         "The following are automatically derived by this proc macro attribute.",
                //     )
                //     .emit();
                panic!(
                    "The following are automatically derived when this attribute is used:\n\
                     {:?}",
                    superfluous
                );
            } else {
                // derives_existing.nested.push_punct(<Token![,]>::default());
                derives_existing.nested.extend(derives_to_append);

                // Replace the existing `Attribute`.
                //
                // `attr.parse_meta()` returns a `Meta`, which is not referenced by the
                // `DeriveInput`, so we have to replace `attr` itself.
                *attr = parse_quote!(#[#derives_existing]);
            }
        } else {
            // Add a new `#[derive(..)]` attribute with all the derives.
            let derive_attribute: Attribute = parse_quote!(#[derive(#derives_to_append)]);
            self.attrs.push(derive_attribute);
        }
    }

    fn contains_tag(&self, namespace: &Path, tag: &Path) -> bool {
        util::contains_tag(&self.attrs, namespace, tag)
    }

    fn tag_parameter(&self, namespace: &Path, tag: &Path) -> Option<NestedMeta> {
        util::tag_parameter(&self.attrs, namespace, tag)
    }

    fn tag_parameters(&self, namespace: &Path, tag: &Path) -> Vec<NestedMeta> {
        util::tag_parameters(&self.attrs, namespace, tag)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use proc_macro2::Span;
    use quote::quote;
    use syn::{parse_quote, DeriveInput, Error, Lit, Meta, NestedMeta};

    use super::DeriveInputExt;

    #[test]
    fn append_derives_creates_attr_when_attr_does_not_exist() {
        let mut ast: DeriveInput = parse_quote!(
            struct Struct;
        );
        let derives = parse_quote!(Clone, Copy);

        ast.append_derives(derives);

        let ast_expected: DeriveInput = parse_quote! {
            #[derive(Clone, Copy)]
            struct Struct;
        };
        assert_eq!(ast_expected, ast);
    }

    #[test]
    fn append_derives_appends_to_attr_when_attr_exists() {
        let mut ast: DeriveInput = parse_quote!(
            #[derive(Debug)]
            struct Struct;
        );
        let derives = parse_quote!(Clone, Copy);

        ast.append_derives(derives);

        let ast_expected: DeriveInput = parse_quote! {
            #[derive(Debug, Clone, Copy)]
            struct Struct;
        };
        assert_eq!(ast_expected, ast);
    }

    #[test]
    #[should_panic(
        expected = "The following are automatically derived when this attribute is used:\n\
                    [\"Clone\", \"Copy\"]"
    )]
    fn append_derives_panics_when_derives_exist() {
        let mut ast: DeriveInput = parse_quote!(
            #[derive(Clone, Copy, Debug)]
            struct Struct;
        );
        let derives = parse_quote!(Clone, Copy, Default);

        ast.append_derives(derives);
    }

    #[test]
    fn contains_tag_returns_true_when_tag_exists() -> Result<(), Error> {
        let ast: DeriveInput = parse_quote!(
            #[my::derive(tag::name)]
            struct Struct;
        );

        assert!(ast.contains_tag(&parse_quote!(my::derive), &parse_quote!(tag::name)));

        Ok(())
    }

    #[test]
    fn contains_tag_returns_false_when_tag_does_not_exist() -> Result<(), Error> {
        let tokens_list = vec![
            quote! {
                #[my::derive]
                struct Struct;
            },
            quote! {
                #[my::derive(other)]
                struct Struct;
            },
            quote! {
                #[other(tag::name)]
                struct Struct;
            },
        ];

        tokens_list
            .into_iter()
            .try_for_each(|tokens| -> Result<(), Error> {
                let message = format!("Failed to parse tokens: `{}`", &tokens);
                let assertion_message = format!(
                    "Expected `contains_tag` to return false for tokens: `{}`",
                    &tokens
                );

                let ast: DeriveInput =
                    syn::parse2(tokens).map_err(|_| Error::new(Span::call_site(), &message))?;

                assert!(
                    !ast.contains_tag(&parse_quote!(my::derive), &parse_quote!(tag::name)),
                    assertion_message // kcov-ignore
                );

                Ok(())
            })
    }

    #[test]
    fn tag_parameter_returns_none_when_not_present() {
        let ast: DeriveInput = parse_quote!(
            #[my::derive]
            struct Struct;
        );

        assert_eq!(
            ast.tag_parameter(&parse_quote!(my::derive), &parse_quote!(tag::name)),
            None
        );
    }

    #[test]
    fn tag_parameter_returns_ident_when_present() {
        let ast: DeriveInput = parse_quote!(
            #[my::derive(tag::name(Magic))]
            struct Struct;
        );

        assert_eq!(
            ast.tag_parameter(&parse_quote!(my::derive), &parse_quote!(tag::name)),
            Some(NestedMeta::Meta(Meta::Path(parse_quote!(Magic))))
        );
    }

    #[test]
    #[should_panic(
        expected = "Expected exactly one identifier for `#[my::derive(tag::name(..))]`."
    )]
    fn tag_parameter_panics_when_multiple_parameters_present() {
        let ast: DeriveInput = parse_quote!(
            #[my::derive(tag::name(Magic::One, Magic::Two))]
            struct Struct;
        );

        ast.tag_parameter(&parse_quote!(my::derive), &parse_quote!(tag::name));
    }

    #[test]
    fn tag_parameters_returns_empty_vec_when_not_present() {
        let ast: DeriveInput = parse_quote!(
            #[my::derive]
            struct Struct;
        );

        assert_eq!(
            ast.tag_parameters(&parse_quote!(my::derive), &parse_quote!(tag::name)),
            Vec::<NestedMeta>::new()
        );
    }

    #[test]
    fn tag_parameters_returns_idents_when_present() {
        let ast: DeriveInput = parse_quote!(
            #[my::derive(tag::name(Magic::One, "{ Magic::Two }"))]
            struct Struct;
        );

        assert_eq!(
            ast.tag_parameters(&parse_quote!(my::derive), &parse_quote!(tag::name)),
            vec![
                NestedMeta::Meta(Meta::Path(parse_quote!(Magic::One))),
                NestedMeta::Lit(Lit::Str(parse_quote!("{ Magic::Two }"))),
            ]
        );
    }
}
