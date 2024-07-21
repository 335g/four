use convert_case::Casing;
use syn::spanned::Spanned;

#[proc_macro_derive(ManagedResource, attributes(resource_type))]
pub fn managed_resource(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    eprintln!("INPUT: {:#?}", input);
    let resource_type = match get_resource_type(&input) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let struct_name = &input.ident;
    let attributes = &input.attrs;

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

    let expnaded = quote::quote! {
        impl serde::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeMap as _;
                use four_core::resource::ManagedResource;

                struct Inner1<'a> {
                    #(#inner_fields),*
                }

                impl serde::Serialize for Inner1<'_> {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        let mut map = serializer.serialize_map(None)?;
                        #(#inner_fields_impl_serialize)*
                        map.end()
                    }
                }

                struct Inner2<'a> {
                    r#type: &'static str,
                    properties: Inner1<'a>,
                }

                impl serde::Serialize for Inner2<'_> {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        let mut map = serializer.serialize_map(Some(2))?;
                        map.serialize_entry("Type", &self.r#type)?;
                        map.serialize_entry("Properties", &self.properties)?;
                        map.end()
                    }
                }

                let properties = Inner1 {
                    #(#inner_fields_init),*
                };
                let inner = Inner2 { r#type: self.resource_type(), properties };

                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(self.logical_id(), &inner)?;
                map.end()
            }
        }

        impl four_core::logical_id::LogicalIdentified for #struct_name {
            fn logical_id(&self) -> &four_core::logical_id::LogicalId {
                &self.logical_id
            }
        }

        impl four_core::resource::ManagedResource for #struct_name {
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
