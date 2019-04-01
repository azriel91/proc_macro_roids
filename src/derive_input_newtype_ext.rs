use syn::{Data, DataStruct, DeriveInput, Field, Fields};

const NEWTYPE_MUST_HAVE_ONLY_ONE_FIELD: &str =
    "Newtype struct must only have one field.\n\
     See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#advanced-types \
     for more information.";
const MACRO_MUST_BE_USED_ON_NEWTYPE_STRUCT: &str =
    "This macro must be used on a newtype struct.\n\
     See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#advanced-types \
     for more information.";

/// Functions to make it ergonomic to work with newtype `struct` ASTs.
pub trait DeriveInputNewtypeExt {
    /// Returns the `Field` of the first unnamed field of this struct's AST.
    ///
    /// # Panics
    ///
    /// Panics if the AST is not for a newtype struct.
    fn inner_type(&self) -> &Field;

    /// Returns the `Field` of the first unnamed field of this struct's AST.
    ///
    /// # Panics
    ///
    /// Panics if the AST is not for a newtype struct.
    fn inner_type_mut(&mut self) -> &mut Field;
}

impl DeriveInputNewtypeExt for DeriveInput {
    fn inner_type(&self) -> &Field {
        if let Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields_unnamed),
            ..
        }) = &self.data
        {
            if fields_unnamed.unnamed.len() == 1 {
                fields_unnamed
                    .unnamed
                    .first()
                    .expect("Expected field to exist.")
                    .value()
            } else {
                panic!(NEWTYPE_MUST_HAVE_ONLY_ONE_FIELD)
            }
        } else {
            panic!(MACRO_MUST_BE_USED_ON_NEWTYPE_STRUCT)
        }
    }

    fn inner_type_mut(&mut self) -> &mut Field {
        if let Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields_unnamed),
            ..
        }) = &mut self.data
        {
            if fields_unnamed.unnamed.len() == 1 {
                fields_unnamed
                    .unnamed
                    .iter_mut()
                    .next()
                    .expect("Expected field to exist.")
            } else {
                panic!(NEWTYPE_MUST_HAVE_ONLY_ONE_FIELD)
            }
        } else {
            panic!(MACRO_MUST_BE_USED_ON_NEWTYPE_STRUCT)
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, DeriveInput, Type};

    use super::DeriveInputNewtypeExt;

    #[test]
    fn inner_type_returns_field() {
        let ast: DeriveInput = parse_quote! {
            struct Newtype(u32);
        };

        let inner_field = ast.inner_type();

        let expected_type: Type = Type::Path(parse_quote!(u32));
        assert_eq!(expected_type, inner_field.ty);
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a newtype struct.\n\
        See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#advanced-types \
        for more information.")]
    fn inner_type_panics_when_struct_fields_not_unnamed() {
        let ast: DeriveInput = parse_quote! {
            struct Unit;
        };

        ast.inner_type();
    }

    #[test]
    #[should_panic(expected = "Newtype struct must only have one field.\n\
        See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#advanced-types \
        for more information.")]
    fn inner_type_panics_when_struct_has_multiple_fields() {
        let ast: DeriveInput = parse_quote! {
            struct Newtype(u32, u32);
        };

        ast.inner_type();
    }

    #[test]
    fn inner_type_mut_returns_field() {
        let mut ast: DeriveInput = parse_quote! {
            struct Newtype(u32);
        };

        let inner_field = ast.inner_type_mut();

        let expected_type: Type = Type::Path(parse_quote!(u32));
        assert_eq!(expected_type, inner_field.ty);
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a newtype struct.\n\
        See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#advanced-types \
        for more information.")]
    fn inner_type_mut_panics_when_struct_fields_not_unnamed() {
        let mut ast: DeriveInput = parse_quote! {
            struct Unit;
        };

        ast.inner_type_mut();
    }

    #[test]
    #[should_panic(expected = "Newtype struct must only have one field.\n\
        See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#advanced-types \
        for more information.")]
    fn inner_type_mut_panics_when_struct_has_multiple_fields() {
        let mut ast: DeriveInput = parse_quote! {
            struct Newtype(u32, u32);
        };

        ast.inner_type_mut();
    }
}
