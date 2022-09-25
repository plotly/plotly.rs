#![recursion_limit = "256"]

use syn::DeriveInput;

mod field_setter;

#[proc_macro_derive(FieldSetter, attributes(field_setter))]
pub fn field_setter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<DeriveInput>(input).unwrap();
    field_setter::field_setter_impl(input)
}
