use proc_macro::TokenStream;
use crate::args::Args;
use crate::expand::expand;
use crate::parse::Item;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn async_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let mut item = parse_macro_input!(input as Item);
    expand(&mut item, args.local);
    TokenStream::from(quote!(#item))
}