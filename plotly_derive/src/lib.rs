#![recursion_limit = "256"]

use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{DeriveInput, GenericArgument, Generics, TypeParamBound};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(field_setter), supports(struct_named))]
struct StructReceiver {
    /// The struct name.
    ident: syn::Ident,

    /// The body of the struct or enum. We don't care about enum fields
    /// because we accept only named structs. Hence the first type is null.
    data: ast::Data<(), FieldReceiver>,

    generics: Generics,

    #[darling(default)]
    no_box: bool,
}

impl ToTokens for StructReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let mut setter_functions = quote![];
        let mut default_vals = quote![];

        match self.data {
            ast::Data::Struct(ref f) => {
                for field in f.fields.iter() {
                    let this_setter = field.setter_function(self.no_box);
                    setter_functions = quote![
                        #setter_functions
                        #this_setter
                    ];

                    let this_default = field.default_value();
                    default_vals = quote![
                        #default_vals
                        #this_default
                    ];
                }
            }
            _ => unreachable!(),
        }

        tokens.append_all(quote! {
            impl #impl_generics Default for #ident #ty_generics #where_clause {
                fn default() -> Self {
                    Self {
                        #default_vals
                    }
                }
            }

            impl #impl_generics #ident #ty_generics #where_clause {
                #setter_functions
            }
        });
    }
}

// Not using a recursive enum for simplicity
#[derive(Clone, Copy)]
enum FieldType {
    NotOption,
    OptionDimString,
    OptionDimOther,
    OptionVecString,
    OptionBoxColor,
    OptionVecBoxColor,
    OptionString,
    OptionNumOrString,
    OptionNumOrStringCollection,
    OptionOther,
}

fn _type_str_parts(field_ty: &syn::Type) -> Vec<String> {
    let mut ty = field_ty;
    let mut parts = Vec::new();
    loop {
        match ty {
            syn::Type::Path(ref type_path) if type_path.qself == None => {
                if let Some(segment) = type_path.path.segments.last() {
                    parts.push(segment.ident.to_string());
                    match &segment.arguments {
                        syn::PathArguments::AngleBracketed(args) => match args.args.first() {
                            Some(first) => {
                                if let GenericArgument::Type(inner_ty) = first {
                                    ty = inner_ty
                                } else {
                                    break;
                                }
                            }
                            None => break,
                        },
                        _ => break,
                    }
                } else {
                    break;
                }
            }
            // Handle Box<dyn Color>
            syn::Type::TraitObject(ref obj) => {
                if obj.dyn_token.is_some() {
                    if let Some(TypeParamBound::Trait(t)) = obj.bounds.first() {
                        if let Some(segment) = t.path.segments.last() {
                            parts.push(segment.ident.to_string());
                        }
                    }
                }
                break;
            }
            _ => break,
        }
    }
    parts
}

impl FieldType {
    fn infer(field_ty: &syn::Type) -> Self {
        // Not the best implementation but works in practice

        let type_str_parts = _type_str_parts(field_ty);
        if type_str_parts.first().map_or(false, |t| t != "Option") {
            return FieldType::NotOption;
        }

        let remaining: Vec<_> = type_str_parts.iter().skip(1).map(|x| x.as_str()).collect();

        match remaining.as_slice() {
            ["Dim", "String"] => FieldType::OptionDimString,
            ["Dim", ..] => FieldType::OptionDimOther,
            ["Vec", "String"] => FieldType::OptionVecString,
            ["String"] => FieldType::OptionString,
            ["NumOrString"] => FieldType::OptionNumOrString,
            ["NumOrStringCollection"] => FieldType::OptionNumOrStringCollection,
            ["Box", "Color"] => FieldType::OptionBoxColor,
            ["Vec", "Box", "Color"] => FieldType::OptionVecBoxColor,
            _ => FieldType::OptionOther,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromField)]
#[darling(attributes(field_setter), forward_attrs(doc))]
struct FieldReceiver {
    /// Name of the field
    ident: Option<syn::Ident>,

    /// The type of the field
    ty: syn::Type,

    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    skip: bool,

    #[darling(default)]
    default: Option<String>,
}

impl FieldReceiver {
    fn default_value(&self) -> TokenStream {
        let field_ident = self.ident.as_ref().unwrap();
        match FieldType::infer(&self.ty) {
            FieldType::NotOption => {
                // Require a default
                assert!(
                    self.default.is_some(),
                    "Please provide #[field_setter(default=\"..\") for the field {}",
                    field_ident
                );
                let val: proc_macro2::TokenStream = self.default.as_ref().unwrap().parse().unwrap();
                quote![
                    #field_ident: #val,
                ]
            }
            _ => {
                quote![
                    #field_ident: None,
                ]
            }
        }
    }
    fn setter_function(&self, no_box: bool) -> TokenStream {
        if self.skip {
            println!("{:#?}", _type_str_parts(&self.ty));
            return quote![];
        }
        let field_ident = self.ident.as_ref().unwrap();
        let field_ty = &self.ty;
        let field_docs = self.docs();
        let (return_ty, return_stmt) = if no_box {
            (quote![Self], quote![self])
        } else {
            (quote![Box<Self>], quote![Box::new(self)])
        };

        let field_type = FieldType::infer(field_ty);

        let (value_type, value_convert, array_value_convert) = match field_type {
            FieldType::NotOption => {
                return quote![];
            }
            FieldType::OptionDimString => (
                quote![impl AsRef<str>],
                quote![Dim::Scalar(value.as_ref().to_owned())],
                quote![Dim::Vector(
                    value.into_iter().map(|v| v.as_ref().to_owned()).collect()
                )],
            ),
            FieldType::OptionDimOther => (
                quote![<<#field_ty as crate::GetInner>::Inner as crate::GetInner>::Inner],
                quote![Dim::Scalar(value)],
                quote![Dim::Vector(value)],
            ),
            FieldType::OptionString => (
                quote![impl AsRef<str>],
                quote![value.as_ref().to_owned()],
                quote![],
            ),
            FieldType::OptionOther => (
                quote![<#field_ty as crate::GetInner>::Inner],
                quote![value],
                quote![],
            ),
            FieldType::OptionVecString => (
                quote![Vec<impl AsRef<str>>],
                quote![value.into_iter().map(|v| v.as_ref().to_owned()).collect()],
                quote![],
            ),
            FieldType::OptionBoxColor => (
                quote![impl crate::color::Color],
                quote![Box::new(value)],
                quote![],
            ),
            FieldType::OptionVecBoxColor => (
                quote![Vec<impl crate::color::Color>],
                quote![crate::color::ColorArray(value).into()],
                quote![],
            ),
            FieldType::OptionNumOrString => (
                quote![impl Into<crate::private::NumOrString>],
                quote![value.into()],
                quote![],
            ),
            FieldType::OptionNumOrStringCollection => (
                quote![Vec<impl Into<crate::private::NumOrString> + Clone>],
                quote![value.into()],
                quote![],
            ),
        };

        let setter = quote! {
            #field_docs
            pub fn #field_ident(mut self, value: #value_type) -> #return_ty {
                self.#field_ident = Some(#value_convert);
                #return_stmt
            }
        };

        let array_setter = match field_type {
            FieldType::OptionDimString | FieldType::OptionDimOther => {
                let array_ident = Ident::new(
                    &format!("{}_array", field_ident),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    #field_docs
                    pub fn #array_ident(mut self, value: Vec<#value_type>) -> #return_ty {
                        self.#field_ident = Some(#array_value_convert);
                        #return_stmt
                    }
                }
            }
            _ => quote![],
        };
        quote![
            #setter
            #array_setter
        ]
    }
    fn docs(&self) -> TokenStream {
        self.attrs
            .iter()
            .filter(|attr| {
                attr.path
                    .segments
                    .first()
                    .map_or(false, |p| p.ident == "doc")
            })
            .map(|attr| {
                quote![
                    #attr
                ]
            })
            .collect()
    }
}

const UNSUPPORTED_ERROR: &str = r#"FieldSetter can only be derived for structs with named fields"#;

#[proc_macro_derive(FieldSetter, attributes(field_setter))]
pub fn html_template(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = syn::parse::<DeriveInput>(item).unwrap();
    let struct_receiver = match StructReceiver::from_derive_input(&item) {
        Ok(r) => r,
        Err(e) => {
            return proc_macro::TokenStream::from(
                darling::Error::custom(format!("{}. {}", UNSUPPORTED_ERROR, e)).write_errors(),
            )
        }
    };
    quote! {
        #struct_receiver
    }
    .into()
}
