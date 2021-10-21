use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use pipeline::expand_execute;

mod pipeline;
mod util;

#[proc_macro_derive(Execute, attributes(handler))]
pub fn derive_helper_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_execute(input).into()
}
