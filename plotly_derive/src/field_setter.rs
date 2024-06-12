use darling::{ast, FromDeriveInput, FromField, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{DeriveInput, GenericArgument, Generics, TypeGenerics, TypeParamBound};

const UNSUPPORTED_ERROR: &str = r#"FieldSetter can only be derived for structs with named fields"#;

pub(crate) fn field_setter_impl(input: DeriveInput) -> proc_macro::TokenStream {
    let struct_receiver = match StructReceiver::from_derive_input(&input) {
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

#[derive(Debug, Clone, Copy, FromMeta, Default)]
#[darling(default)]
enum Kind {
    #[default]
    Other,
    Layout,
    Trace,
}

impl Kind {
    fn name_prefix(self) -> &'static str {
        match self {
            Kind::Other => unreachable!(),
            Kind::Layout => "Relayout",
            Kind::Trace => "Restyle",
        }
    }
}

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
    box_self: bool,

    #[darling(default)]
    kind: Kind,
}

impl ToTokens for StructReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let mut setter_functions = quote![];
        let mut default_vals = quote![];
        let mut enum_variants = quote![];

        match self.data {
            ast::Data::Struct(ref f) => {
                for field in f.fields.iter() {
                    let (s, e) = field.tokens(self.box_self, self.kind, &self.ident, &ty_generics);
                    setter_functions.append_all(s);
                    enum_variants.append_all(e);
                    default_vals.append_all(field.default_value());
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

        let (enum_ident, trait_ident) = match self.kind {
            Kind::Other => {
                return;
            }
            Kind::Layout => (
                Ident::new(
                    &format!("Relayout{}", self.ident),
                    proc_macro2::Span::call_site(),
                ),
                quote![crate::Relayout],
            ),
            Kind::Trace => (
                Ident::new(
                    &format!("Restyle{}", self.ident),
                    proc_macro2::Span::call_site(),
                ),
                quote![crate::Restyle],
            ),
        };

        tokens.append_all(quote! {
            #[derive(Serialize, Clone)]
            #[serde(untagged)]
            #[allow(clippy::enum_variant_names)]
            pub enum #enum_ident #ty_generics #where_clause {
                #enum_variants
            }

            impl #impl_generics #trait_ident for #enum_ident #ty_generics #where_clause {}
        })
    }
}

// Not using a recursive enum for simplicity
#[derive(Clone)]
enum FieldType {
    NotOption,
    OptionDimString,
    OptionDimOther(syn::Type),
    OptionVecString,
    OptionBoxColor,
    OptionVecBoxColor,
    OptionBoxOther(syn::Type),
    OptionString,
    OptionNumOrString,
    OptionNumOrStringCollection,
    OptionOther(syn::Type),
}

fn _type_str_parts(field_ty: &syn::Type) -> (Vec<String>, Vec<syn::Type>) {
    let mut ty = field_ty;
    let mut parts = Vec::new();
    let mut types = Vec::new();
    types.push(ty.clone());
    loop {
        match ty {
            syn::Type::Path(ref type_path) if type_path.qself.is_none() => {
                if let Some(segment) = type_path.path.segments.last() {
                    parts.push(segment.ident.to_string());
                    match &segment.arguments {
                        syn::PathArguments::AngleBracketed(args) => match args.args.first() {
                            Some(first) => {
                                if let GenericArgument::Type(inner_ty) = first {
                                    ty = inner_ty;
                                    types.push(ty.clone());
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
    (parts, types)
}

impl FieldType {
    fn inner_type_quote(&self) -> TokenStream {
        match self {
            FieldType::NotOption => unreachable!(),
            FieldType::OptionDimString => quote![crate::common::Dim<String>],
            FieldType::OptionDimOther(inner) => quote![crate::common::Dim<#inner>],
            FieldType::OptionVecString => quote![Vec<String>],
            FieldType::OptionBoxColor => quote![Box<dyn crate::color::Color>],
            FieldType::OptionVecBoxColor => quote![Vec<Box<dyn crate::color::Color>>],
            FieldType::OptionString => quote![String],
            FieldType::OptionNumOrString => quote![crate::private::NumOrString],
            FieldType::OptionNumOrStringCollection => quote![crate::private::NumOrStringCollection],
            FieldType::OptionOther(inner) => quote![#inner],
            FieldType::OptionBoxOther(inner) => quote![Box<#inner>],
        }
    }

    fn infer(field_ty: &syn::Type) -> Self {
        // Not the best implementation but works in practice

        let (type_str_parts, types) = _type_str_parts(field_ty);
        if type_str_parts.first().map_or(false, |t| t != "Option") {
            return FieldType::NotOption;
        }

        let remaining: Vec<_> = type_str_parts.iter().skip(1).map(|x| x.as_str()).collect();

        match remaining.as_slice() {
            ["Dim", "String"] => FieldType::OptionDimString,
            ["Dim", ..] => FieldType::OptionDimOther(types.get(2).cloned().unwrap()),
            ["Vec", "String"] => FieldType::OptionVecString,
            ["String"] => FieldType::OptionString,
            ["NumOrString"] => FieldType::OptionNumOrString,
            ["NumOrStringCollection"] => FieldType::OptionNumOrStringCollection,
            ["Box", "Color"] => FieldType::OptionBoxColor,
            ["Box", ..] => FieldType::OptionBoxOther(types.get(2).cloned().unwrap()),
            ["Vec", "Box", "Color"] => FieldType::OptionVecBoxColor,
            _ => FieldType::OptionOther(types.get(1).cloned().unwrap()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromField)]
#[darling(attributes(field_setter), forward_attrs(doc, serde))]
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

    fn tokens(
        &self,
        box_self: bool,
        kind: Kind,
        struct_ident: &Ident,
        ty_generics: &TypeGenerics,
    ) -> (TokenStream, TokenStream) {
        if self.skip {
            return (quote![], quote![]);
        }
        let field_ident = self.ident.as_ref().unwrap();
        let field_ty = &self.ty;
        let field_docs = self.docs();
        let (return_ty, return_stmt) = if box_self {
            (quote![Box<Self>], quote![Box::new(self)])
        } else {
            (quote![Self], quote![self])
        };

        let field_type = FieldType::infer(field_ty);

        let (value_type, value_convert, array_value_convert) = match &field_type {
            FieldType::NotOption => {
                return (quote![], quote![]);
            }
            FieldType::OptionDimString => (
                quote![impl AsRef<str>],
                quote![crate::common::Dim::Scalar(value.as_ref().to_owned())],
                quote![crate::common::Dim::Vector(
                    value.into_iter().map(|v| v.as_ref().to_owned()).collect()
                )],
            ),
            FieldType::OptionDimOther(inner_ty) => (
                quote![#inner_ty],
                quote![crate::common::Dim::Scalar(value)],
                quote![crate::common::Dim::Vector(value)],
            ),
            FieldType::OptionString => (
                quote![impl AsRef<str>],
                quote![value.as_ref().to_owned()],
                quote![],
            ),
            FieldType::OptionOther(inner_ty) => (quote![#inner_ty], quote![value], quote![]),
            FieldType::OptionVecString => (
                quote![Vec<impl AsRef<str>>],
                quote![value.into_iter().map(|v| v.as_ref().to_owned()).collect()],
                quote![],
            ),
            FieldType::OptionBoxColor => (
                quote![impl crate::color::Color],
                quote![Box::new(value) as Box<dyn crate::color::Color>],
                quote![],
            ),
            FieldType::OptionBoxOther(inner_ty) => {
                (quote![#inner_ty], quote![Box::new(value)], quote![])
            }
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

        struct ModifyEnum {
            variant_name: Ident,
            enum_name: Ident,
            enum_variant: TokenStream,
        }

        let modify_enum = match kind {
            Kind::Other => None,
            Kind::Layout | Kind::Trace => {
                let mut variant_name = String::new();
                let mut capitalize = true;
                for ch in field_ident.to_string().chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        variant_name.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        variant_name.push(ch);
                    }
                }
                let variant_name = Ident::new(
                    &format!("Modify{}", variant_name),
                    proc_macro2::Span::call_site(),
                );
                let serde_attrs = self.serde();
                let inner_ty = field_type.inner_type_quote();
                Some(ModifyEnum {
                    enum_name: Ident::new(
                        &format!("{}{}", kind.name_prefix(), struct_ident),
                        proc_macro2::Span::call_site(),
                    ),
                    enum_variant: if matches!(kind, Kind::Trace) {
                        quote! {
                            #variant_name {
                                #serde_attrs
                                #field_ident: Option<crate::common::Dim<#inner_ty>>
                            },
                        }
                    } else {
                        quote! {
                            #variant_name {
                                #serde_attrs
                                #field_ident: Option<#inner_ty>
                            },
                        }
                    },
                    variant_name,
                })
            }
        };

        let mut setter = quote! {
            #field_docs
            pub fn #field_ident(mut self, value: #value_type) -> #return_ty {
                self.#field_ident = Some(#value_convert);
                #return_stmt
            }
        };

        if let Some(ModifyEnum {
            variant_name,
            enum_name,
            ..
        }) = &modify_enum
        {
            let modify_ident = Ident::new(
                &format!("modify_{}", field_ident),
                proc_macro2::Span::call_site(),
            );
            match kind {
                Kind::Trace => {
                    let modify_all_ident = Ident::new(
                        &format!("modify_all_{}", field_ident),
                        proc_macro2::Span::call_site(),
                    );

                    setter.append_all(quote![
                        /// Apply the same restyling to all the traces
                        pub fn #modify_all_ident(value: #value_type) -> #enum_name #ty_generics {
                            #enum_name::#variant_name {
                                #field_ident: Some(crate::common::Dim::Scalar(#value_convert))
                            }
                        }
                        /// Apply the restyling individually to each trace. Caller is responsible to set the length of the vector
                        /// to be equal to the number of traces
                        pub fn #modify_ident(values: Vec<#value_type>) -> #enum_name #ty_generics {
                            #enum_name::#variant_name {
                                #field_ident: Some(crate::common::Dim::Vector(values.into_iter().map(|value| #value_convert).collect()))
                            }
                        }
                    ]);
                }
                Kind::Layout => {
                    setter.append_all(quote![
                        pub fn #modify_ident(value: #value_type) -> #enum_name #ty_generics {
                            #enum_name::#variant_name {
                                #field_ident: Some(#value_convert)
                            }
                        }
                    ]);
                }
                Kind::Other => unreachable!(),
            }
        }

        let array_setter = match field_type {
            FieldType::OptionDimString | FieldType::OptionDimOther(..) => {
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

        let enum_variant = modify_enum.map(|m| m.enum_variant).unwrap_or_default();

        (
            quote![
                #setter
                #array_setter
            ],
            enum_variant,
        )
    }

    fn docs(&self) -> TokenStream {
        self.search_attrs("doc")
    }

    fn serde(&self) -> TokenStream {
        self.search_attrs("serde")
    }

    fn search_attrs(&self, name: &str) -> TokenStream {
        self.attrs
            .iter()
            .filter(|attr| {
                attr.path()
                    .segments
                    .first()
                    .map_or(false, |p| p.ident == name)
            })
            .map(|attr| {
                quote![
                    #attr
                ]
            })
            .collect()
    }
}
