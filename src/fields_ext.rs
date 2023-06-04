use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed, Ident};

/// Functions to make it ergonomic to work with `Fields`.
pub trait FieldsExt {
    /// Returns true if the `Fields` is for a unit struct.
    fn is_unit(&self) -> bool;

    /// Returns true if the `Fields` is for a struct with named fields.
    fn is_named(&self) -> bool;

    /// Returns true if the `Fields` is for a struct with unnamed fields.
    fn is_tuple(&self) -> bool;

    /// Returns a token stream of the construction form of the fields.
    ///
    /// For unit fields, this returns an empty token stream.
    ///
    /// * Tuple fields: `(_0, _1,)`
    /// * Named fields: `{ field_0, field_1 }`
    ///
    /// # Examples
    fn construction_form(&self) -> TokenStream;
}

impl FieldsExt for Fields {
    fn is_unit(&self) -> bool {
        matches!(self, Fields::Unit)
    }

    fn is_named(&self) -> bool {
        matches!(self, Fields::Named(..))
    }

    fn is_tuple(&self) -> bool {
        matches!(self, Fields::Unnamed(..))
    }

    fn construction_form(&self) -> TokenStream {
        match self {
            Fields::Unit => TokenStream::new(),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let token_stream =
                    (0..unnamed.len()).fold(TokenStream::new(), |mut token_stream, n| {
                        let tuple_field = Ident::new(format!("_{}", n).as_str(), Span::call_site());
                        token_stream.extend(quote!(#tuple_field, ));
                        token_stream
                    });

                quote! { (#token_stream) }
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let token_stream = named.iter().filter_map(|field| field.ident.as_ref()).fold(
                    TokenStream::new(),
                    |mut token_stream, field_name| {
                        token_stream.extend(quote!(#field_name, ));
                        token_stream
                    },
                );

                quote!({ #token_stream })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::{parse_quote, Fields, FieldsNamed, FieldsUnnamed};

    use super::FieldsExt;

    #[test]
    fn is_unit_returns_true_when_fields_unit() {
        assert!(Fields::Unit.is_unit());
    }

    #[test]
    fn is_unit_returns_false_when_fields_not_unit() {
        let fields_named: FieldsNamed = parse_quote! {{}};
        let fields = Fields::from(fields_named);

        assert!(!fields.is_unit());
    }

    #[test]
    fn is_named_returns_true_when_fields_named() {
        let fields_named: FieldsNamed = parse_quote! {{}};
        let fields = Fields::from(fields_named);

        assert!(fields.is_named());
    }

    #[test]
    fn is_named_returns_false_when_fields_not_named() {
        assert!(!Fields::Unit.is_named());
    }

    #[test]
    fn is_tuple_returns_true_when_fields_unnamed() {
        let fields_unnamed: FieldsUnnamed = parse_quote! {(u32,)};
        let fields = Fields::from(fields_unnamed);

        assert!(fields.is_tuple());
    }

    #[test]
    fn is_tuple_returns_false_when_fields_not_unnamed() {
        assert!(!Fields::Unit.is_tuple());
    }

    #[test]
    fn construction_form_fields_unit_is_empty_token_stream() {
        assert!(Fields::Unit.construction_form().is_empty());
    }

    #[test]
    fn construction_form_fields_named_is_brace_surrounding_comma_separated_variable_names() {
        let fields_named: FieldsNamed = parse_quote! {{
            pub field_0: u32,
            pub field_1: SomeType,
        }};
        let fields = Fields::from(fields_named);
        let construction_tokens = fields.construction_form();

        let expected_tokens = quote!({ field_0, field_1, });
        assert_eq!(expected_tokens.to_string(), construction_tokens.to_string());
    }

    #[test]
    fn construction_form_fields_unnamed_is_parentheses_surrounding_comma_separated_variable_ns() {
        let fields_unnamed: FieldsUnnamed = parse_quote! {(u32, u32)};
        let fields = Fields::from(fields_unnamed);
        let construction_tokens = fields.construction_form();

        let expected_tokens = quote!((_0, _1,));
        assert_eq!(expected_tokens.to_string(), construction_tokens.to_string());
    }

    #[test]
    fn construction_form_fields_unnamed_one_field_includes_trailing_comma() {
        let fields_unnamed: FieldsUnnamed = parse_quote! {(u32,)};
        let fields = Fields::from(fields_unnamed);
        let construction_tokens = fields.construction_form();

        let expected_tokens = quote!((_0,));
        assert_eq!(expected_tokens.to_string(), construction_tokens.to_string());
    }
}
