# Changelog

## 0.5.0 (2019-08-19)

* `syn`, `quote`, and `proc_macro2` are upgraded to `1.0`.
* ***Breaking:*** `nested_meta_to_ident` is renamed to `nested_meta_to_path`.

## 0.4.0 (2019-08-17)

* ***Breaking:*** `DeriveInputDeriveExt` is renamed to `DeriveInputExt`.
* `FieldExt::tag_parameter` extracts the `Meta` param from `#[namespace(tag(param))]`.
* `FieldExt::tag_parameters` extracts the `Meta` params from `#[namespace(tag(param1, param2))]`.
* `DeriveInputExt::tag_parameter` extracts the `Meta` param from `#[namespace(tag(param))]`.
* `DeriveInputExt::tag_parameters` extracts the `Meta` params from `#[namespace(tag(param1, param2))]`.

## 0.3.0 (2019-08-04)

* `FieldExt` provides methods to work with `Field`s:

    - `contains_tag`
    - `is_phantom_data`
    - `type_name`

## 0.2.1 (2019-04-10)

* `IdentExt::append` and `IdentExt::prepend` create new `Ident`s via concatenation.
* Added the following methods to `DeriveInputStructExt`:

    - `is_unit`
    - `is_named`
    - `is_tuple`
    - `assert_fields_unit`
    - `assert_fields_named`
    - `assert_fields_unnamed`

* Added `is_newtype` to `DeriveInputNewtypeExt`.

## 0.2.0 (2019-04-02)

* ***Breaking:*** `FieldsNamed::append` is renamed to `FieldsNamed::append_named`.
* ***Breaking:*** `FieldsUnnamed::append` is renamed to `FieldsUnnamed::append_unnamed`.

## 0.1.0 (2019-04-01)

* `DeriveInputDeriveExt` provides function to append `derive`s.
* `DeriveInputNewtypeExt` provides functions to get newtype inner `Field`.
* `DeriveInputStructExt` provides functions to get struct `Field`s.
* `FieldsNamedAppend` provides functions to append `FieldsNamed`.
* `FieldsUnnamedAppend` provides functions to append `FieldsUnnamed`.
* `nested_meta_to_ident` returns the `Ident` of a nested meta.
* `meta_list_contains` returns whether a `MetaList` contains a specified `NestedMeta`.
* `ident_concat` returns an `Ident` by concatenating `String` representations.
