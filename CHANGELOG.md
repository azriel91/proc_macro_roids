# Changelog

## 0.2.1 (unreleased)

* `IdentExt#append` and `#prepend` creates new `Ident`s via concatenation.
* Added the following methods to `DeriveInputStructExt`:

    - `is_unit`
    - `is_named`
    - `is_tuple`
    - `assert_fields_unit`
    - `assert_fields_named`
    - `assert_fields_unnamed`

* Added `is_newtype` to `DeriveInputNewtypeExt`.

## 0.2.0 (2019-04-02)

* ***Breaking:*** `FieldsNamed#append` is renamed to `FieldsNamed#append_named`.
* ***Breaking:*** `FieldsUnnamed#append` is renamed to `FieldsUnnamed#append_unnamed`.

## 0.1.0 (2019-04-01)

* `DeriveInputDeriveExt` provides function to append `derive`s.
* `DeriveInputNewtypeExt` provides functions to get newtype inner `Field`.
* `DeriveInputStructExt` provides functions to get struct `Field`s.
* `FieldsNamedAppend` provides functions to append `FieldsNamed`.
* `FieldsUnnamedAppend` provides functions to append `FieldsUnnamed`.
* `nested_meta_to_ident` returns the `Ident` of a nested meta.
* `meta_list_contains` returns whether a `MetaList` contains a specified `NestedMeta`.
* `ident_concat` returns an `Ident` by concatenating `String` representations.
