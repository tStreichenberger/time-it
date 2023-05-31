extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use quote::ToTokens;
use syn::{
    parse::{
        Parse,
        ParseStream,
    },
    parse_macro_input,
    Block,
    Ident,
    Item::Verbatim,
    LitStr,
    Result as SynResult,
    Stmt,
    Token,
};

#[proc_macro]
pub fn time_it(to_time: TokenStream) -> TokenStream {
    let to_time = parse_macro_input!(to_time as TimeItInput);

    let tag_opt = match to_time.tag {
        Some(ref tag) => Verbatim(quote!(Some(#tag))),
        None => Verbatim(quote!(None)),
    };

    let start_ident = get_random_name();

    quote! {
        let #start_ident = std::time::Instant::now();
        #to_time
        let duration = #start_ident.elapsed();
        time_it::action(#tag_opt, duration);
    }
    .into()
}

#[proc_macro_attribute]
pub fn time_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as syn::ItemFn);
    let name = (&func.sig.ident).to_string();
    let name = name.as_str();

    let timer_name = get_random_name();

    let start_timer_tokens =
        quote! { let #timer_name = ::time_it::Timer(Some(#name), ::std::time::Instant::now()); }
            .into();
    let start_time_stmt = parse_macro_input!(start_timer_tokens as Stmt);

    func.block.stmts.insert(0, start_time_stmt);

    quote! {
        #func
    }
    .into()
}

/// generates a random identifier to be used as the start instant name to avoid overwritting any other vars
fn get_random_name() -> Ident {
    let random_name = uuid::Uuid::new_v4().to_string().replace("-", "");

    Ident::new(
        &format!("_start_{}", random_name),
        proc_macro::Span::call_site().into(),
    )
}

struct TimeItInput {
    tag: Option<LitStr>,
    stmts: Vec<Stmt>,
}

impl Parse for TimeItInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut tag = None;

        let stmts = if input.peek(LitStr) && input.peek2(Token![,]) {
            tag = Some(input.parse()?);
            input.parse::<Token![,]>()?;
            let block = input.parse::<Block>()?;
            block.stmts
        } else {
            input.call(Block::parse_within)?
        };

        Ok(TimeItInput { tag, stmts })
    }
}

impl ToTokens for TimeItInput {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        self.stmts.iter().for_each(|stmt| stmt.to_tokens(tokens))
    }
}
