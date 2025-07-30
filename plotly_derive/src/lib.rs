#![recursion_limit = "256"]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::DeriveInput;
use syn::{parse_macro_input, ItemStruct};
mod field_setter;

#[proc_macro_derive(FieldSetter, attributes(field_setter))]
pub fn field_setter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<DeriveInput>(input).unwrap();
    field_setter::field_setter_impl(input)
}

#[proc_macro_attribute]
pub fn layout_structs(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let vis = &input.vis;
    let fields = &input.fields;
    let attrs = &input.attrs;

    // Generate LayoutTemplate (no template field, no #[field_setter(kind =
    // "layout")])
    let template_struct_name = format_ident!("LayoutTemplate");
    let template_struct = quote! {
        #(#attrs)*
        #[serde_with::skip_serializing_none]
        #[derive(Serialize, Debug, Clone, FieldSetter)]
        #vis struct #template_struct_name #fields
    };

    // Generate Layout (with template field and #[field_setter(kind = "layout")])
    let layout_struct_name = format_ident!("Layout");
    let mut layout_fields = fields.clone();
    // Add the template field
    if let syn::Fields::Named(ref mut named) = layout_fields {
        named.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    #[field_setter(skip)]
                    template: Option<Box<std::borrow::Cow<'static, Template>>>
                })
                .unwrap(),
        );
    }
    let layout_struct = quote! {
        #(#attrs)*
        #[serde_with::skip_serializing_none]
        #[derive(Serialize, Debug, Clone, FieldSetter)]
        #[field_setter(kind = "layout")]
        #vis struct #layout_struct_name #layout_fields
    };

    // Generate impls (add your methods as needed)
    let template_impl = quote! {
        impl #template_struct_name {
            pub fn new() -> Self { Default::default() }
            pub fn add_annotation(&mut self, annotation: Annotation) {
                if self.annotations.is_none() { self.annotations = Some(Vec::new()); }
                self.annotations.as_mut().unwrap().push(annotation);
            }
            pub fn add_shape(&mut self, shape: Shape) {
                if self.shapes.is_none() { self.shapes = Some(Vec::new()); }
                self.shapes.as_mut().unwrap().push(shape);
            }
        }
    };
    let layout_impl = quote! {
        impl #layout_struct_name {
            pub fn new() -> Self { Default::default() }
            pub fn to_json(&self) -> String { serde_json::to_string(self).unwrap() }
            pub fn add_annotation(&mut self, annotation: Annotation) {
                if self.annotations.is_none() { self.annotations = Some(Vec::new()); }
                self.annotations.as_mut().unwrap().push(annotation);
            }
            pub fn add_shape(&mut self, shape: Shape) {
                if self.shapes.is_none() { self.shapes = Some(Vec::new()); }
                self.shapes.as_mut().unwrap().push(shape);
            }
            pub fn template<T>(mut self, template: T) -> Self
            where T: Into<std::borrow::Cow<'static, Template>> {
                self.template = Some(Box::new(template.into()));
                self
            }
        }
    };

    let expanded = quote! {
        #template_struct
        #layout_struct
        #template_impl
        #layout_impl
    };
    TokenStream::from(expanded)
}
