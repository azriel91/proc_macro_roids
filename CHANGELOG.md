# Changelog

## 0.8.0 (2023-06-04)

syn 2 upgrade. All of the changes are **Breaking**, and so have not been highlighted as such.

* `DeriveInputExt::append_derives` parameters changed from `NestedMeta` to `syn::Path`.
* `DeriveInputExt::tag_parameter` return type changed from `NestedMeta` to `Meta`.
* `DeriveInputExt::tag_parameters` return type changed from `NestedMeta` to `Meta`.
* `util::nested_meta_to_path` is removed.
* `util::meta_list_contains` is removed.
* Removed `util::namespace_meta_lists` -- use `util::namespace_nested_metas_iter`.
* Replaced `util::namespace_meta_lists_iter` with `util::namespace_nested_metas_iter`.
* Replaced `util::tag_meta_lists_iter` with `util::tag_nested_metas_iter`.
* Removed `util::tag_meta_lists_owned_iter`, there is no borrowed version because of `syn`'s new API, so use `util::tag_nested_metas_iter`.
* Removed `util::ident_concat` -- users can use `quote::format_ident!` instead.
* Added `util::namespace_parameter` and `util::namespace_parameters`.


## 0.7.0 (2020-01-13)

* `util::namespace_parameter` returns an `Option<NestedMeta>`.
* `util::namespace_parameters` returns a `Vec<NestedMeta>`.
* ***Breaking:*** `util::namespace_meta_lists_iter` returns an `impl Iterator<Item = MetaList>`.
* ***Breaking:*** `util::tag_meta_list` renamed to `util::tag_meta_lists_iter`.
* ***Breaking:*** `util::tag_meta_list_owned` renamed to `util::tag_meta_lists_owned_iter`.
* ***Breaking:*** `util::tag_meta_lists_owned_iter` takes in `impl Iterator<Item = MetaList>` instead of `Vec<MetaList>`.

## 0.6.1 (2020-01-10)

* `util::contains_tag` supports checking if any list of attributes contains a `#[namespace(tag)]`.
* `DeriveInputExt::contains_tag` supports checking if a type contains a `#[namespace(tag)]`.
* Added `FieldsExt::is_unit/is_named/is_tuple` which returns a `bool` for the relevant `Fields` type.
* `FieldsExt::construction_form` returns tokens suitable for deconstructing / constructing the relevant fields types.

## 0.6.0 (2019-10-01)

* ***Breaking:*** `DeriveInputExt::tag_parameter` and `DeriveInputExt::tag_parameters` return `NestedMeta`.
* ***Breaking:*** `FieldExt::tag_parameter` and `FieldExt::tag_parameters` return `NestedMeta`.
* `util::tag_parameter` and `util::tag_parameters` are now `pub`.
* `util::namespace_meta_list` is now `pub`.
* `util::tag_meta_list` and `util::tag_meta_list_owned` are now `pub`.

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
