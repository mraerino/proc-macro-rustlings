extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_first_proc_macro(_item: TokenStream) -> TokenStream {
    todo!()
}
