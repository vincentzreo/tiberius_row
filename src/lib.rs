use proc_macro::TokenStream;
use rowext::process_row_ext;

mod rowext;

#[proc_macro_derive(Row_Ext)]
pub fn derive_row_ext(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_row_ext(input).into()
}
