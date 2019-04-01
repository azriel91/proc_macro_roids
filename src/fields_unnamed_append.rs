use syn::{DeriveInput, Fields, FieldsUnnamed};

use crate::DeriveInputStructExt;

const ERR_MUST_BE_UNIT_OR_UNNAMED: &str =
    "Macro must be used on either a unit struct or tuple struct.\n\
     This derive does not work on structs with named fields.";

/// Indicates this type may have `FieldsUnnamed` appended to it.
pub trait FieldsUnnamedAppend {
    /// Appends the specified `fields_unnamed` to this type.
    fn append(&mut self, fields_unnamed: FieldsUnnamed);
}

impl FieldsUnnamedAppend for DeriveInput {
    fn append(&mut self, fields_unnamed: FieldsUnnamed) {
        self.fields_mut().append(fields_unnamed);
    }
}

impl FieldsUnnamedAppend for Fields {
    fn append(&mut self, fields_unnamed: FieldsUnnamed) {
        match self {
            Fields::Named(_) => panic!(ERR_MUST_BE_UNIT_OR_UNNAMED),
            Fields::Unit => *self = Fields::from(fields_unnamed),
            Fields::Unnamed(self_fields_unnamed) => self_fields_unnamed.append(fields_unnamed),
        }
    }
}

impl FieldsUnnamedAppend for FieldsUnnamed {
    fn append(&mut self, fields_unnamed: FieldsUnnamed) {
        self.unnamed.extend(fields_unnamed.unnamed);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use syn::{parse_quote, DeriveInput, Fields, FieldsUnnamed};

    use super::FieldsUnnamedAppend;

    #[test]
    fn append_fields_unnamed_to_fields_unnamed() {
        let mut fields: FieldsUnnamed = parse_quote!((u32, i32));
        let fields_additional: FieldsUnnamed = parse_quote!((i64, usize));
        let fields_expected: FieldsUnnamed = parse_quote!((u32, i32, i64, usize));

        fields.append(fields_additional);

        assert_eq!(fields_expected, fields);
    }

    #[test]
    fn append_fields_unnamed_to_fields_unit() {
        let mut fields = Fields::Unit;
        let fields_additional: FieldsUnnamed = parse_quote!((i64, usize));
        let fields_expected: Fields = Fields::Unnamed(parse_quote!((i64, usize)));

        fields.append(fields_additional);

        assert_eq!(fields_expected, fields);
    }

    #[test]
    #[should_panic(
        expected = "Macro must be used on either a unit struct or tuple struct.\n\
                    This derive does not work on structs with named fields."
    )]
    fn append_fields_unnamed_to_fields_unnamed_panics() {
        let mut fields: Fields = Fields::Named(parse_quote!({ a: u32, b: i32 }));
        let fields_additional: FieldsUnnamed = parse_quote!((i64, usize));

        fields.append(fields_additional);
    }

    #[test]
    fn append_fields_unnamed_to_struct_unnamed() {
        let mut ast: DeriveInput = parse_quote! {
            struct StructUnnamed(u32, i32);
        };

        let fields_additional: FieldsUnnamed = parse_quote!((i64, usize));
        ast.append(fields_additional);

        let ast_expected: DeriveInput = parse_quote! {
            struct StructUnnamed(u32, i32, i64, usize);
        };
        assert_eq!(ast_expected, ast);
    }

    #[test]
    fn append_fields_unnamed_to_struct_unit() {
        let mut ast: DeriveInput = parse_quote! {
            struct StructUnit;
        };

        let fields_additional: FieldsUnnamed = parse_quote!((i64, usize));
        ast.append(fields_additional);

        let ast_expected: DeriveInput = parse_quote! {
            struct StructUnit(i64, usize);
        };
        assert_eq!(ast_expected, ast);
    }
}
