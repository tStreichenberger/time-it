extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro]
pub fn time_it(to_time: TokenStream) -> TokenStream {
    let to_time = syn::Item::Verbatim(to_time.into());

    quote! {
        let start = std::time::Instant::now();
        #to_time
        println!("Operation took {}ms", start.elapsed().as_millis())
    }
    .into()
}
