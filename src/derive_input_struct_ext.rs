use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

/// Functions to make it ergonomic to work with `struct` ASTs.
pub trait DeriveInputStructExt {
    /// Returns a reference to the data_struct of a struct's AST.
    ///
    /// # Panics
    ///
    /// Panics if the AST is not for a struct.
    fn data_struct(&self) -> &DataStruct;

    /// Returns a mutable reference to the data_struct of a struct's AST.
    ///
    /// # Panics
    ///
    /// Panics if the AST is not for a struct.
    fn data_struct_mut(&mut self) -> &mut DataStruct;

    /// Returns a reference to the fields of a struct's AST.
    ///
    /// # Panics
    ///
    /// Panics if the AST is not for a struct.
    fn fields(&self) -> &Fields;

    /// Returns a mutable reference to the fields of a struct's AST.
    ///
    /// # Panics
    ///
    /// Panics if the AST is not for a struct.
    fn fields_mut(&mut self) -> &mut Fields;

    /// Returns a mutable reference to the named fields of a struct's AST.
    ///
    /// # Panics
    ///
    /// Panics if the AST is not for a struct with named fields.
    fn fields_named(&self) -> &FieldsNamed;

    /// Returns a mutable reference to the named fields of a struct's AST.
    ///
    /// # Panics
    ///
    /// Panics if the AST is not for a struct with named fields.
    fn fields_named_mut(&mut self) -> &mut FieldsNamed;
}

impl DeriveInputStructExt for DeriveInput {
    fn data_struct(&self) -> &DataStruct {
        if let Data::Struct(data_struct) = &self.data {
            data_struct
        } else {
            panic!("This macro must be used on a struct.");
        }
    }

    fn data_struct_mut(&mut self) -> &mut DataStruct {
        if let Data::Struct(data_struct) = &mut self.data {
            data_struct
        } else {
            panic!("This macro must be used on a struct.");
        }
    }

    fn fields(&self) -> &Fields {
        if let Data::Struct(DataStruct { fields, .. }) = &self.data {
            fields
        } else {
            panic!("This macro must be used on a struct.");
        }
    }

    fn fields_mut(&mut self) -> &mut Fields {
        if let Data::Struct(DataStruct { fields, .. }) = &mut self.data {
            fields
        } else {
            panic!("This macro must be used on a struct.");
        }
    }

    fn fields_named(&self) -> &FieldsNamed {
        if let Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) = &self.data
        {
            fields_named
        } else {
            panic!("This macro must be used on a struct with named fields.");
        }
    }

    fn fields_named_mut(&mut self) -> &mut FieldsNamed {
        if let Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) = &mut self.data
        {
            fields_named
        } else {
            panic!("This macro must be used on a struct with named fields.");
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, DeriveInput, Fields, FieldsNamed};

    use super::DeriveInputStructExt;

    #[test]
    fn data_struct_returns_data_struct() {
        let ast: DeriveInput = parse_quote! {
            struct Unit;
        };

        ast.data_struct();
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a struct.")]
    fn data_struct_panics_when_ast_is_not_struct() {
        let ast: DeriveInput = parse_quote! {
            enum NotStruct {}
        };

        ast.data_struct();
    }

    #[test]
    fn data_struct_mut_returns_data_struct_mut() {
        let mut ast: DeriveInput = parse_quote! {
            struct Unit;
        };

        ast.data_struct_mut();
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a struct.")]
    fn data_struct_mut_panics_when_ast_is_not_struct() {
        let mut ast: DeriveInput = parse_quote! {
            enum NotStruct {}
        };

        ast.data_struct_mut();
    }

    #[test]
    fn fields_returns_unit_fields() {
        let ast: DeriveInput = parse_quote! {
            struct Unit;
        };

        assert_eq!(&Fields::Unit, ast.fields());
    }

    #[test]
    fn fields_returns_named_fields() {
        let ast: DeriveInput = parse_quote! {
            struct Named {}
        };

        if let &Fields::Named(..) = ast.fields() {
            // pass
        } else {
            panic!("Expected `fields` to return `&Fields::Named(..)")
        }
    }

    #[test]
    fn fields_returns_unnamed_fields() {
        let ast: DeriveInput = parse_quote! {
            struct Unnamed(u32);
        };

        if let &Fields::Unnamed(..) = ast.fields() {
            // pass
        } else {
            panic!("Expected `fields` to return `&Fields::Unnamed(..)")
        }
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a struct.")]
    fn fields_panics_when_ast_is_not_struct() {
        let ast: DeriveInput = parse_quote! {
            enum NotStruct {}
        };

        ast.fields();
    }

    #[test]
    fn fields_mut_returns_unit_fields() {
        let mut ast: DeriveInput = parse_quote! {
            struct Unit;
        };

        assert_eq!(&mut Fields::Unit, ast.fields_mut());
    }

    #[test]
    fn fields_mut_returns_named_fields() {
        let mut ast: DeriveInput = parse_quote! {
            struct Named {}
        };

        if let &mut Fields::Named(..) = ast.fields_mut() {
            // pass
        } else {
            panic!("Expected `fields_mut` to return `&mut Fields::Named(..)")
        }
    }

    #[test]
    fn fields_mut_returns_unnamed_fields() {
        let mut ast: DeriveInput = parse_quote! {
            struct Unnamed(u32);
        };

        if let &mut Fields::Unnamed(..) = ast.fields_mut() {
            // pass
        } else {
            panic!("Expected `fields_mut` to return `&mut Fields::Unnamed(..)")
        }
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a struct.")]
    fn fields_mut_panics_when_ast_is_not_struct() {
        let mut ast: DeriveInput = parse_quote! {
            enum NotStruct {}
        };

        ast.fields_mut();
    }

    #[test]
    fn fields_named_returns_named_fields() {
        let ast: DeriveInput = parse_quote! {
            struct Named { a: u32, b: i32 }
        };

        let fields_named: FieldsNamed = parse_quote!({ a: u32, b: i32 });
        assert_eq!(&fields_named, ast.fields_named());
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a struct with named fields.")]
    fn fields_named_panics_when_fields_unit() {
        let ast: DeriveInput = parse_quote! {
            struct Unit;
        };

        ast.fields_named();
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a struct with named fields.")]
    fn fields_named_panics_when_ast_is_not_struct() {
        let ast: DeriveInput = parse_quote! {
            enum NotStruct {}
        };

        ast.fields_named();
    }

    #[test]
    fn fields_named_mut_returns_named_fields() {
        let mut ast: DeriveInput = parse_quote! {
            struct Named { a: u32, b: i32 }
        };

        let mut fields_named: FieldsNamed = parse_quote!({ a: u32, b: i32 });
        assert_eq!(&mut fields_named, ast.fields_named_mut());
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a struct with named fields.")]
    fn fields_named_mut_panics_when_fields_unit() {
        let mut ast: DeriveInput = parse_quote! {
            struct Unit;
        };

        ast.fields_named_mut();
    }

    #[test]
    #[should_panic(expected = "This macro must be used on a struct with named fields.")]
    fn fields_named_mut_panics_when_ast_is_not_struct() {
        let mut ast: DeriveInput = parse_quote! {
            enum NotStruct {}
        };

        ast.fields_named_mut();
    }
}
