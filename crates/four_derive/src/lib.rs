use convert_case::Casing;
use quote::quote;
use syn::{spanned::Spanned, PathArguments};

#[proc_macro_derive(ManagedResource, attributes(four, resource_type, referenced))]
pub fn managed_resource(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    eprintln!("INPUT: {:#?}", input);
    let resource_type = match get_resource_type(&input) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let struct_name = &input.ident;

    let syn::Data::Struct(syn::DataStruct {
        struct_token: _,
        fields,
        semi_token: _,
    }) = &input.data
    else {
        return syn::Error::new(input.span(), "").to_compile_error().into();
    };
    let syn::Fields::Named(syn::FieldsNamed {
        brace_token: _,
        named,
    }) = fields
    else {
        return syn::Error::new(input.span(), "").to_compile_error().into();
    };
    let inner_fields = get_inner_fields(named);
    let inner_fields_init = get_inner_fields_init(named);
    let inner_fields_impl_serialize = get_inner_fields_impl_serialize(named);
    let initializer_props = get_initializer_props(named);
    let initializer_values = get_initializer_values(named);
    let setter = get_setter(named);

    let expnaded = quote::quote! {
        impl #struct_name {
            pub fn new(#(#initializer_props),*) -> #struct_name {
                #struct_name {
                    #(#initializer_values),*
                }
            }

            #(#setter)*
        }

        impl serde::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeMap as _;
                use four::logical_id::LogicalIdentified as _;
                use four::resource::ManagedResource as _;

                struct Inner<'a> {
                    #(#inner_fields),*
                }

                impl serde::Serialize for Inner<'_> {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        let mut map = serializer.serialize_map(None)?;
                        #(#inner_fields_impl_serialize)*
                        map.end()
                    }
                }

                let properties = Inner {
                    #(#inner_fields_init),*
                };

                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("Type", &self.resource_type())?;
                map.serialize_entry("Properties", &properties)?;
                map.end()
            }
        }

        impl four::logical_id::LogicalIdentified for #struct_name {
            fn logical_id(&self) -> &four::logical_id::LogicalId {
                &self.logical_id
            }
        }

        impl four::resource::ManagedResource for #struct_name {
            fn resource_type(&self) -> &'static str {
                #resource_type
            }
        }
    };

    proc_macro::TokenStream::from(expnaded)
}

fn get_resource_type(input: &syn::DeriveInput) -> Result<String, syn::Error> {
    if input.attrs.is_empty() {
        return Err(syn::Error::new(
            input.ident.span(),
            "must use 1 `resource_type` attribute to define resource type of resource",
        ));
    }

    let values = input
        .attrs
        .iter()
        .map(|attr| attr.meta.clone())
        .filter(|meta| meta.path().is_ident("resource_type"))
        .map(|meta| meta.require_name_value().map(|s| s.clone()))
        .collect::<Result<Vec<_>, _>>()?;

    if values.len() != 1 {
        return Err(syn::Error::new(
            input.ident.span(),
            "must use 1 `resource_type` attribute to define resource type of resource",
        ));
    }

    let value = values.first().expect("have 1 resource_type").clone();

    let syn::Expr::Lit(syn::ExprLit { attrs: _, lit }) = &value.value else {
        return Err(syn::Error::new(
            input.ident.span(),
            "`resouce_type` attribute can only accept string literal",
        ));
    };

    let syn::Lit::Str(s) = lit else {
        return Err(syn::Error::new(
            input.ident.span(),
            "`resouce_type` attribute can only accept string literal",
        ));
    };

    Ok(s.value())
}

fn get_inner_fields<'a>(
    fields: impl IntoIterator<Item = &'a syn::Field>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .into_iter()
        .filter(|f| f.ident.as_ref().map(|i| i.to_string()) != Some("logical_id".to_string()))
        .filter(|f| {
            f.attrs
                .iter()
                .filter(|attr| is_skip_attribute(*attr))
                .next()
                .is_none()
        })
        .map(|f| {
            let name = &f.ident;
            let ty = &f.ty;

            quote::quote! {
                #name: &'a #ty
            }
            .into()
        })
        .collect()
}

fn get_inner_fields_init<'a>(
    fields: impl IntoIterator<Item = &'a syn::Field>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .into_iter()
        .filter(|f| f.ident.as_ref().map(|i| i.to_string()) != Some("logical_id".to_string()))
        .filter(|f| {
            f.attrs
                .iter()
                .filter(|attr| is_skip_attribute(*attr))
                .next()
                .is_none()
        })
        .map(|f| {
            let name = &f.ident;

            quote::quote! {
                #name: &self.#name
            }
            .into()
        })
        .collect()
}

fn get_inner_fields_impl_serialize<'a>(
    fields: impl IntoIterator<Item = &'a syn::Field>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .into_iter()
        .filter(|f| f.ident.as_ref().map(|i| i.to_string()) != Some("logical_id".to_string()))
        .filter(|f| {
            f.attrs
                .iter()
                .filter(|attr| is_skip_attribute(*attr))
                .next()
                .is_none()
        })
        .map(|f| {
            let name = &f.ident;
            let key = name
                .as_ref()
                .map(|s| s.to_string().to_case(convert_case::Case::Pascal));

            if is_option(&f.ty) {
                quote::quote! {
                    if let Some(#name) = &self.#name {
                        map.serialize_entry(#key, #name)?;
                    }
                }
                .into()
            } else {
                quote::quote! {
                    map.serialize_entry(#key, &self.#name)?;
                }
                .into()
            }
        })
        .collect()
}

fn is_option(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };

    type_path
        .path
        .segments
        .last()
        .map(|seg| seg.ident == "Option")
        .unwrap_or_default()
}

/// #[four(skip)]
fn is_skip_attribute(attribute: &syn::Attribute) -> bool {
    let syn::Meta::List(list) = &attribute.meta else {
        return false;
    };

    if !list.path.is_ident("four") {
        return false;
    }

    format!("{}", list.tokens) == "skip"
}

fn get_initializer_props<'a>(
    fields: impl IntoIterator<Item = &'a syn::Field>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .into_iter()
        .filter(|f| !is_option(&f.ty))
        .map(|f| {
            let name = &f.ident;
            let ty = &f.ty;

            quote! {
                #name: #ty
            }
        })
        .collect()
}

fn get_initializer_values<'a>(
    fields: impl IntoIterator<Item = &'a syn::Field>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .into_iter()
        .map(|f| {
            let name = &f.ident;

            if is_option(&f.ty) {
                quote! { #name: None }
            } else {
                quote! { #name }
            }
        })
        .collect()
}

fn get_setter<'a>(
    fields: impl IntoIterator<Item = &'a syn::Field>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .into_iter()
        .filter(|f| f.ident.as_ref().map(|i| i.to_string()) != Some("logical_id".to_string()))
        .filter(|f| is_option(&f.ty))
        .map(|f| {
            let name = &f.ident;
            let ty = &f.ty;
            let syn::Type::Path(path) = &f.ty else {
                return quote! {};
            };
            let Some(seg) = path.path.segments.first() else {
                return quote! {};
            };
            let PathArguments::AngleBracketed(bra) = &seg.arguments else {
                return quote! {};
            };
            let Some(syn::GenericArgument::Type(ty)) = bra.args.first() else {
                return quote! {};
            };

            quote! {
                fn #name(mut self, #name: #ty) -> Self {
                    self.#name = Some(#name);
                    self
                }
            }
        })
        .collect()
}
