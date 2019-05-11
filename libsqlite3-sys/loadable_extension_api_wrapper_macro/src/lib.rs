extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro]
pub fn loadable_extension_api_wrapper(input: TokenStream) -> TokenStream {
    println!("loadable_extension_api_wrapper: input = {:?}", input);
    //let ast = syn::parse(input).expect(&format!("loadable_extension_api_wrapper macro failed to parse input TokenStream"));
    //impl_loadable_extension_api_wrapper(&ast)
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

fn impl_loadable_extension_api_wrapper(ast: &syn::DeriveInput) -> TokenStream {
   let name = &ast.ident;
   println!("impl_loadable_extension_api_wrapper: name = {}", name);
   let gen = quote! {
     fn answer() -> u32 { 42 }
   };
   gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
