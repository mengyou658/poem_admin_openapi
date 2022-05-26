use std::error::Error;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(QueryObject, attributes(app_derive))]
pub fn derive_object(input: proc_macro::TokenStream) -> TokenStream {
  let args = parse_macro_input!(input as DeriveInput);
  generate(args).unwrap_or_else(to_compile_errors)
    .into()
}


pub(crate) fn generate(args: DeriveInput) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
  let ident = &args.ident;

  let define_obj = quote! {
        impl poem_openapi::types::ParseFromParameter for #ident {
            fn parse_from_parameter(_value: &str) -> poem_openapi::types::ParseResult<Self> {
                poem_openapi::types::ParseResult::Ok(serde_qs::from_str(_value)?)
            }
        }
    };

  Ok(quote! {
        #define_obj
    })
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
  let compile_errors = errors.iter().map(syn::Error::to_compile_error);
  quote!(#(#compile_errors)*)
}


#[cfg(test)]
mod tests {



    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
