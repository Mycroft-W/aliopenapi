extern crate proc_macro;
use proc_macro::TokenStream;

/// Derive macro generating an impl of the trait `Api`
#[proc_macro_derive(Api)]
pub fn api_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let struct_identifier = input.ident;

    let expanded = quote::quote! {
        impl Api for #struct_identifier {
            fn new() -> Self {
                let parameters = OrderMap::new();
                #struct_identifier(parameters)
            }

            fn name(&self) -> String {
                stringify!(#struct_identifier).to_string()
            }

            fn send(self) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> {
                RequestHeader::new(
                    super::ENDPOINT.to_string(),
                    self.name(),
                    super::VERSION.to_string(),
                    self.0,
                )
                .sign()
                .send()
            }
        }
    };
    expanded.into()
}


