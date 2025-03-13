use quote::quote;
use syn::{DeriveInput, Type};
use utils::get_struct_name;

mod utils;

#[proc_macro_derive(AutoNewBuilder)]
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
    let mut build_stream = Vec::<proc_macro2::TokenStream>::new();

    if let syn::Data::Struct(data_struct) = data {

        let mut has_optional_fields_started = false;
    
        for field in data_struct.fields {
            let field_name = field.clone().ident.unwrap();

            
    
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
                                // output_stream
                                //     .push(format_option_encode(field_name, tlv_config).unwrap());

                                field_stream.push(
                                    quote!{
                                        #field_name : None
                                    }
                                );

                                let with_method_name = syn::Ident::new(
                                    &format!("with_{}", field_name),
                                    field_name.span()
                                );

                                with_fn_stream.push(
                                    quote! {
                                        #[inline]
                                        pub fn #with_method_name(mut self, value: #inner_type) -> Self {
                                            self.#field_name = Some(value);
                                            self
                                        }
                                    }
                                );

                                build_stream.push(
                                    quote! {}
                                );
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

            new_stream.push(
                quote!{
                    #field_name: #field_type
                }
            );

            field_stream.push(
                quote!{
                    #field_name : #field_name
                }
            );
            
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