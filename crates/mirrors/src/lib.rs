#![doc = include_str!("../../../README.md")]

#[proc_macro_derive(FieldNames, attributes(field_names))]
/// Derive `FieldNames` trait to extract struct field names as a slice.
///
///
/// ```
///use mirrors::FieldNames;
///
///#[derive(FieldNames)]
///struct Foo {
///    f0: i32,
///    #[field_names(rename = "f1")]
///    f_1: i32,
///    f_n: i32,
/// }
///
/// assert_eq!(Foo::field_names(),["f0","f1","f_n"])
/// ```
///
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mirrors_internal::derive_field_names(input.into()).into()
}
