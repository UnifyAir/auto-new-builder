use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident};


pub fn get_struct_name(struct_stream: TokenStream) -> Ident {
	let input = syn::parse2::<DeriveInput>(struct_stream.clone()).unwrap();
	match input.data {
		syn::Data::Struct(_) => input.ident,
		_ => {
            panic!("This derive macro only works on structs");
        },
	}
}