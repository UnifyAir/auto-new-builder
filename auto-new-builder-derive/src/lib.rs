use quote::quote;
use syn::{DataStruct, DeriveInput, Ident, Type};
use utils::get_struct_name;

mod utils;

#[proc_macro_derive(AutoNewBuilder, attributes(auto_new_value, auto_new_required))]
pub fn auto_new_builder_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input: proc_macro2::TokenStream = syn::parse_macro_input!(input);
    let output_stream: proc_macro2::TokenStream = generate(parsed_input);
    output_stream.into()
}

fn generate_struct(struct_name: Ident, data_struct: DataStruct) -> proc_macro2::TokenStream {
    let mut new_stream = Vec::<proc_macro2::TokenStream>::new();
    let mut field_stream = Vec::<proc_macro2::TokenStream>::new();
    let mut with_fn_stream = Vec::<proc_macro2::TokenStream>::new();

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
                if let Some(last_segment) = type_path.path.segments.last() {
                    if last_segment.ident == "Option" {
                        if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                            if args.args.len() == 1 {
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
                    } else {
                        let field_type = type_path.path;
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
                        } else {
                            new_stream.push(quote! {
                                #field_name: #field_type
                            });

                            field_stream.push(quote! {
                                #field_name : #field_name
                            });
                        }
                    }
                }
            }
            _ => {
                panic!("Unsupported type in generic");
            }
        };
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
}

fn generate_newtype(struct_name: Ident, data_struct: DataStruct) -> proc_macro2::TokenStream {
    let mut new_stream = Vec::<proc_macro2::TokenStream>::new();
    let mut field_stream = Vec::<proc_macro2::TokenStream>::new();
    let mut with_fn_stream = Vec::<proc_macro2::TokenStream>::new();

    let mut auto_new_value = None;
    let mut auto_new_required = false;

    if let syn::Fields::Unnamed(fields) = &data_struct.fields {
        let field = fields.unnamed[0].clone();
        let field_attr = field.clone().attrs;

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
                if let Some(last_segment) = type_path.path.segments.last() {
                    if last_segment.ident == "Option" {
                        if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                            if args.args.len() == 1 {
                                let inner_type = &args.args[0];

                                if auto_new_value.is_some() {
                                    field_stream.push(quote! {
                                        #auto_new_value
                                    });
                                } else if auto_new_required {
                                    new_stream.push(quote! {
                                        inner: #inner_type
                                    });
                                    field_stream.push(quote! {
                                        Some(inner)
                                    });
                                } else {
                                    field_stream.push(quote! {
                                        None
                                    });
                                }

                                with_fn_stream.push(quote! {
                                    #[inline]
                                    pub fn with_inner(mut self, value: #inner_type) -> Self {
                                        self.0 = Some(value);
                                        self
                                    }
                                });
                            } else {
                                panic!("Option must have exactly one type parameter");
                            }
                        } else {
                            panic!("Invalid Option type format");
                        }
                    } else {
                        let field_type = type_path.path;

                        if auto_new_value.is_some() {
                            field_stream.push(quote! {
                                #auto_new_value
                            });
                            with_fn_stream.push(quote! {
                                #[inline]
                                pub fn with_inner(mut self, value: #field_type) -> Self {
                                    self.0 = value;
                                    self
                                }
                            });
                        } else {
                            new_stream.push(quote! {
                                inner: #field_type
                            });
                            field_stream.push(quote! {
                                inner
                            });
                        }
                    }
                }
            }
            _ => {
                panic!("Unsupported type in generic");
            }
        };

        return quote! {
            impl #struct_name {
                pub fn new(#(#new_stream),*) -> Self {
                    #struct_name (#(#field_stream),*)
                }
                #(#with_fn_stream)*
            }
        };
    }
    panic!("Unsupported type");
}

fn generate(input_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let DeriveInput { data, .. } = syn::parse2(input_stream.clone()).unwrap();
    let struct_name = get_struct_name(input_stream.clone());

    if let syn::Data::Struct(data_struct) = data {
        match &data_struct.fields {
            syn::Fields::Named(_) => generate_struct(struct_name, data_struct),
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                generate_newtype(struct_name, data_struct)
            }
            syn::Fields::Unnamed(_) => {
                panic!("Tuple structs with multiple fields are not supported");
            }
            syn::Fields::Unit => {
                panic!("Unit structs are not supported");
            }
        }
    } else {
        panic!("AutoNewBuilder can only be derived for structs");
    }
}
