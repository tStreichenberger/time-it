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
        let duration = start.elapsed();

        let config = time_it::get_config();
        let read = config.read().unwrap();
        match read.action {
            Some(ref action) => action(duration),
            None => println!("No action defined"),
        }
    }
    .into()
}
