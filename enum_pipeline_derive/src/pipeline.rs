use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, ExprMatch};

use crate::util::{AsGeneratedIdent, OfRelevantType};

/// Expands the [derive(Execute)] macro into a pipeline implementation using #[handler(my_func_handler)] helper attributes.
/// Example:
/// ```
/// #[derive(Execute)]
/// pub enum Test {
///     #[handler(handle_one)]
///     One(f32),
///     #[handler(handle_two)]
///     Two,
/// }

/// impl Test {
///     fn handle_one(v: f32) {}

///     fn handle_two() {}
/// }
/// ```
// TODO(bengreenier): Use fewer raw strings to implement this
pub fn expand_execute(input: DeriveInput) -> TokenStream {
    let enum_ident = input.ident;
    let enum_name = enum_ident.to_string();

    let variants = match input.data {
        Data::Enum(e) => e.variants,
        _ => panic!("Pipeline derive macro only works on enums"),
    };

    // get the arms as strings
    let arms: Vec<String> = variants
        .into_iter()
        .map(|variant| {
            let variant_name = variant.ident.to_string();
            let full_variant_name = format!("{}::{}", enum_name, variant_name);
            let variant_handlers_all: Vec<Attribute> = variant.attrs.of_relevant_type("handler");

            // error handling for handler attributes
            match variant_handlers_all.len() {
                0 => panic!(
                    "Variant {} is missing attribute #[handler(your_handler_function)]",
                    full_variant_name
                ),
                l if l > 1 => panic!(
                    "Variant {} has too many handler attributes",
                    full_variant_name
                ),
                _ => (),
            }

            let variant_handler_path = variant_handlers_all[0].tokens.to_string();
            let variant_handler_fn =
                variant_handler_path[1..variant_handler_path.len() - 1].to_string();

            // ensure the full variant handler function is qualified
            let full_variant_handler_fn = match variant_handler_fn.contains("::") {
                true => variant_handler_fn,
                false => format!("{}::{}", enum_name, variant_handler_fn),
            };

            let variant_field_names: Vec<String> = variant.fields.as_generated_ident("__");

            let variant_arm = match variant_field_names.len() {
                // qualified variant name => qualified function call()
                0 => format!("{} => {}()", full_variant_name, full_variant_handler_fn),
                // qualified variant name (inner params) => qualified function call(inner params forwarded)
                _ => format!(
                    "{}({}) => {}({})",
                    full_variant_name,
                    variant_field_names.join(","),
                    full_variant_handler_fn,
                    variant_field_names.join(",")
                ),
            };

            variant_arm
        })
        .collect();

    let contents =
        syn::parse_str::<ExprMatch>(&format!("match self {{\n{}\n}}", arms.join(",\n"))).unwrap();

    let res = quote! {
        #[automatically_derived]
        impl Execute for #enum_ident {
            fn execute(self) {
                #contents
            }
        }
    };

    res
}
