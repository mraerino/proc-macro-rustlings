extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn my_first_proc_macro(item: TokenStream) -> TokenStream {
    println!("{:?}", item);
    let item: proc_macro2::TokenStream = item.into();
    let name = proc_macro2::Literal::string("Prince");
    let tree = proc_macro2::TokenTree::Literal(name);
    let function = quote! {
        fn #item() -> &'static str {
            #tree
        }
    };
    println!("{:?}", function);
    function.into()
}
