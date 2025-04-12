use quote::quote;
use syn::{DeriveInput, Type};
use utils::get_struct_name;

mod utils;

#[proc_macro_derive(AutoNewBuilder, attributes(auto_new_value, auto_new_required))]
pub fn auto_new_builder_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input: proc_macro2::TokenStream = syn::parse_macro_input!(input);
    let output_stream: proc_macro2::TokenStream = generate(parsed_input);
    output_stream.into()
}

fn generate(input_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let DeriveInput { data, .. } = syn::parse2(input_stream.clone()).unwrap();
    let struct_name = get_struct_name(input_stream.clone());

    let mut new_stream = Vec::<proc_macro2::TokenStream>::new();
    let mut field_stream = Vec::<proc_macro2::TokenStream>::new();
    let mut with_fn_stream = Vec::<proc_macro2::TokenStream>::new();

    if let syn::Data::Struct(data_struct) = data {
        let mut has_optional_fields_started = false;

        for field in data_struct.fields {
            let field_name = field.clone().ident.unwrap();
            let field_attr = field.clone().attrs;

            let mut auto_new_value = None;
            let mut auto_new_required = false;

            for attr in field_attr {
                if let syn::Meta::NameValue(meta) = &attr.meta {
                    if meta.path.is_ident("auto_new_value") {
                        match &meta.value {
                            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                                syn::Lit::Str(lit_str) => {
                                    let value_str = lit_str.value();
                                    let tokens = value_str
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("Failed to parse string as token stream");
                                    auto_new_value = Some(tokens);
                                }
                                lit => {
                                    auto_new_value = Some(quote::quote!(#lit));
                                }
                            },
                            _ => panic!("auto_new_value must be a literal"),
                        }
                        break;
                    }
                } else if let syn::Meta::Path(meta) = &attr.meta {
                    if meta.is_ident("auto_new_required") {
                        auto_new_required = true;
                        break;
                    }
                }
            }

            match field.clone().ty {
                Type::Path(type_path) => {
                    if type_path.path.segments.len() == 1
                        && type_path.path.segments[0].ident == "Option"
                    {
                        if let syn::PathArguments::AngleBracketed(args) =
                            &type_path.path.segments[0].arguments
                        {
                            if args.args.len() == 1 {
                                has_optional_fields_started = true;
                                let inner_type = &args.args[0];

                                if auto_new_value.is_some() {
                                    field_stream.push(quote! {
                                        #field_name : #auto_new_value
                                    });
                                } else if auto_new_required {
                                    new_stream.push(quote! {
                                        #field_name : #inner_type
                                    });
                                    field_stream.push(quote! {
                                        #field_name : Some(#field_name)
                                    });
                                } else {
                                    field_stream.push(quote! {
                                        #field_name : None
                                    });
                                }

                                let with_method_name = syn::Ident::new(
                                    &format!("with_{}", field_name),
                                    field_name.span(),
                                );

                                with_fn_stream.push(quote! {
                                    #[inline]
                                    pub fn #with_method_name(mut self, value: #inner_type) -> Self {
                                        self.#field_name = Some(value);
                                        self
                                    }
                                });

                                continue;
                            } else {
                                panic!("Option must have exactly one type parameter");
                            }
                        } else {
                            panic!("Invalid Option type format");
                        }
                    }
                }
                _ => {
                    panic!("Unsupported type in generic");
                }
            };

            if has_optional_fields_started {
                panic!("Optional Fields should be the at the last")
            }

            let field_type = match field.ty {
                Type::Path(type_path) => type_path.path,
                _ => {
                    panic!("Unsupported type in generic");
                }
            };

            if auto_new_value.is_some() {
                field_stream.push(quote! {
                    #field_name : #auto_new_value
                });
                let with_method_name =
                    syn::Ident::new(&format!("with_{}", field_name), field_name.span());

                with_fn_stream.push(quote! {
                    #[inline]
                    pub fn #with_method_name(mut self, value: #field_type) -> Self {
                        self.#field_name = value;
                        self
                    }
                });
                continue;
            }

            new_stream.push(quote! {
                #field_name: #field_type
            });

            field_stream.push(quote! {
                #field_name : #field_name
            });
        }

        quote! {
            impl #struct_name {
                pub fn new(#(#new_stream),*) -> Self {
                    #struct_name {
                        #(#field_stream),*
                    }
                }

                #(#with_fn_stream)*

            }
        }
    } else {
        panic!("AutoNewBuilder can only be derived for structs");
    }
}
