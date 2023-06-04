use syn::{DeriveInput, Fields, FieldsNamed};

use crate::DeriveInputStructExt;

const ERR_MUST_BE_UNIT_OR_NAMED: &str = "Macro must be used on either a unit struct or a struct with named fields.\n\
     This derive does not work on tuple structs.";

/// Indicates this type may have `FieldsNamed` appended to it.
pub trait FieldsNamedAppend {
    /// Appends the specified `fields_named` to this type.
    fn append_named(&mut self, fields_named: FieldsNamed);
}

impl FieldsNamedAppend for DeriveInput {
    fn append_named(&mut self, fields_named: FieldsNamed) {
        self.fields_mut().append_named(fields_named);
        self.data_struct_mut().semi_token = None;
    }
}

impl FieldsNamedAppend for Fields {
    fn append_named(&mut self, fields_named: FieldsNamed) {
        match self {
            Fields::Named(self_fields_named) => self_fields_named.append_named(fields_named),
            Fields::Unit => *self = Fields::from(fields_named),
            Fields::Unnamed(_) => panic!("{}", ERR_MUST_BE_UNIT_OR_NAMED),
        }
    }
}

impl FieldsNamedAppend for FieldsNamed {
    fn append_named(&mut self, fields_named: FieldsNamed) {
        self.named.extend(fields_named.named);
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, DeriveInput, Fields, FieldsNamed};

    use super::FieldsNamedAppend;

    #[test]
    fn append_fields_named_to_fields_named() {
        let mut fields: FieldsNamed = parse_quote!({ a: u32, b: i32 });
        let fields_additional: FieldsNamed = parse_quote!({ c: i64, d: usize });
        let fields_expected: FieldsNamed = parse_quote!({ a: u32, b: i32, c: i64, d: usize });

        fields.append_named(fields_additional);

        assert_eq!(fields_expected, fields);
    }

    #[test]
    fn append_fields_named_to_fields_unit() {
        let mut fields = Fields::Unit;
        let fields_additional: FieldsNamed = parse_quote!({ c: i64, d: usize });
        let fields_expected: Fields = Fields::Named(parse_quote!({ c: i64, d: usize }));

        fields.append_named(fields_additional);

        assert_eq!(fields_expected, fields);
    }

    #[test]
    #[should_panic(
        expected = "Macro must be used on either a unit struct or a struct with named fields.\n\
                    This derive does not work on tuple structs."
    )]
    fn append_fields_named_to_fields_unnamed_panics() {
        let mut fields: Fields = Fields::Unnamed(parse_quote!((u32, i32)));
        let fields_additional: FieldsNamed = parse_quote!({ c: i64, d: usize });

        fields.append_named(fields_additional);
    }

    #[test]
    fn append_fields_named_to_struct_named() {
        let mut ast: DeriveInput = parse_quote! {
            struct StructNamed { a: u32, b: i32 }
        };

        let fields_additional: FieldsNamed = parse_quote!({ c: i64, d: usize });
        ast.append_named(fields_additional);

        let ast_expected: DeriveInput = parse_quote! {
            struct StructNamed { a: u32, b: i32, c: i64, d: usize }
        };
        assert_eq!(ast_expected, ast);
    }

    #[test]
    fn append_fields_named_to_struct_unit() {
        let mut ast: DeriveInput = parse_quote! {
            struct StructUnit;
        };

        let fields_additional: FieldsNamed = parse_quote!({ c: i64, d: usize });
        ast.append_named(fields_additional);

        let ast_expected: DeriveInput = parse_quote! {
            struct StructUnit { c: i64, d: usize }
        };
        assert_eq!(ast_expected, ast);
    }
}
