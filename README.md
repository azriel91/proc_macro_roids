[![Crates.io](https://img.shields.io/crates/v/proc_macro_roids.svg)](https://crates.io/crates/proc_macro_roids)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/azriel91/proc_macro_roids?branch=master&svg=true)](https://ci.appveyor.com/project/azriel91/proc_macro_roids/branch/master)
[![Build Status](https://travis-ci.org/azriel91/proc_macro_roids.svg?branch=master)](https://travis-ci.org/azriel91/proc_macro_roids)

# Proc Macro Roids

Traits and functions to make writing proc macros more ergonomic.

The *roids* name is chosen because, although these functions make it easy to perform certain
operation, they may not necessarily be a good idea =D!

## Examples

1. Append additional `#[derive(..)]`s.

   This works for function-like or attribute proc macros.

    ```rust,edition2018
    use proc_macro_roids::DeriveInputDeriveExt;
    use syn::{parse_quote, DeriveInput};

    # fn main() {
    // This may be parsed from the proc macro token stream.
    let mut ast: DeriveInput = parse_quote! {
        #[derive(Debug)]
        struct Struct;
    };

    // Append the derives.
    let derives = parse_quote!(Clone, Copy);
    ast.append_derives(derives);

    // That's it!
    let ast_expected: DeriveInput = parse_quote! {
        #[derive(Debug, Clone, Copy)]
        struct Struct;
    };
    assert_eq!(ast_expected, ast);
    # }
    ```

2. Append named fields.

    This works for structs with named fields or unit structs.

    ```rust,edition2018
    use proc_macro_roids::FieldsNamedAppend;
    use syn::{parse_quote, DeriveInput, FieldsNamed};

    # fn main() {
    // This may be parsed from the proc macro token stream.
    let mut ast: DeriveInput = parse_quote! {
        struct StructNamed { a: u32, b: i32 }
    };

    // Append the fields.
    let fields_additional: FieldsNamed = parse_quote!({ c: i64, d: usize });
    ast.append(fields_additional);

    // That's it!
    let ast_expected: DeriveInput = parse_quote! {
        struct StructNamed { a: u32, b: i32, c: i64, d: usize }
    };
    assert_eq!(ast_expected, ast);
    # }
    ```

3. Append unnamed fields (tuples).

    This works for structs with unnamed fields or unit structs.

    ```rust,edition2018
    use proc_macro_roids::FieldsUnnamedAppend;
    use syn::{parse_quote, DeriveInput, FieldsUnnamed};

    # fn main() {
    // This may be parsed from the proc macro token stream.
    let mut ast: DeriveInput = parse_quote! {
        struct StructUnit;
    };

    // Append the fields.
    let fields_additional: FieldsUnnamed = parse_quote!((i64, usize));
    ast.append(fields_additional);

    // That's it!
    let ast_expected: DeriveInput = parse_quote! {
        struct StructUnit(i64, usize);
    };
    assert_eq!(ast_expected, ast);
    # }
    ```

4. Get newtype inner `Field`.

    This works for structs with unnamed fields or unit structs.

    ```rust,edition2018
    use proc_macro_roids::DeriveInputNewtypeExt;
    use syn::{parse_quote, DeriveInput, Type};

    # fn main() {
    // This may be parsed from the proc macro token stream.
    let mut ast: DeriveInput = parse_quote! {
        struct Newtype(u32);
    };

    // Get the inner field.
    let inner_field = ast.inner_type_mut();

    // That's it!
    let expected_type: Type = Type::Path(parse_quote!(u32));
    assert_eq!(expected_type, inner_field.ty);
    # }
    ```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
