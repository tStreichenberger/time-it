extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro]
pub fn time_it(to_time: TokenStream) -> TokenStream {
    let to_time = syn::Item::Verbatim(to_time.into());

    let random_name = uuid::Uuid::new_v4().to_string().replace("-", "");

    let start_ident = syn::Ident::new(&format!("start_{}", random_name), proc_macro::Span::call_site().into());

    quote! {
        let #start_ident = std::time::Instant::now();
        #to_time
        println!("Operation took {}ms", #start_ident.elapsed().as_millis())
    }
    .into()
}
