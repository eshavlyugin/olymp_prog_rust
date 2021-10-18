extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields,Data,DataStruct};

#[proc_macro_derive(FromTokenStream)]
pub fn from_token_stream(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_from_token_stream(&ast)
}

fn impl_from_token_stream(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let mut fields_vec_innards = quote!();
    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        fields_vec_innards.extend(quote!(self.#field_name = iter.next().as_ref().unwrap().parse().unwrap();));
    }

    let gen = quote! {
        impl<'a, I: Iterator<Item=&'a String>> FromTokenStream<'a, I> for #name {
            fn parse_from(self: &mut #name, iter: &mut I) {
                #fields_vec_innards
            }
        }
    };
    gen.into()
}
