extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use quote::ToTokens;

use syn::parse::*;
use syn::Item::*;
use syn::Result as SynResult;
use syn::*;

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
        time_it::run(#tag_opt, duration);
    }
    .into()
}

#[proc_macro_attribute]
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = parse(item).unwrap();
    let fn_name = &ast.sig.ident;
    let block = &ast.block;

    let result = quote! {
        fn #fn_name() {
            #block
            // Await the subscriber to finish processing
            time_it::wait_for_finish();
        }
    };

    result.into()
}

// TODO: fix the unused code warning to not reveal inner func
#[proc_macro_attribute]
pub fn time_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut old_fn = parse_macro_input!(item as syn::ItemFn);
    let mut new_fn = old_fn.clone();

    // Rename existing function to something that should not have any collisions
    let old_name = old_fn.sig.ident.clone();
    let mut old_name_string = old_name.to_string();
    old_name_string.push_str("_time_it_fn_inner");
    let hidden_name = Ident::new(
        old_name_string.as_str(),
        proc_macro::Span::call_site().into(),
    );
    old_fn.sig.ident = hidden_name.clone();

    // parse out the inputs of the function so we can call them in the new function
    let inputs_with_types = old_fn.sig.inputs.clone();
    let just_args: syn::punctuated::Punctuated<syn::Pat, syn::token::Comma> = inputs_with_types
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(pat_typed) => *pat_typed.pat.clone(),
            syn::FnArg::Receiver(..) => panic!("#[time_fn] does not support methods"), //TODO: add support for this
        })
        .collect();

    // look for and remove lifetime from generics since we won't use it to call inner func
    // let mut found_lifetime = None;
    let mut generics = old_fn.sig.generics.clone();
    generics.params = generics
        .params
        .into_iter()
        .filter(|gen| match gen {
            syn::GenericParam::Lifetime(_) => {
                // found_lifetime = Some(lifetime.clone());
                false
            }
            _ => true,
        })
        .collect();

    // fetch generic arguments to use them to call hidden function
    let (_, type_generics, _) = generics.split_for_impl();
    let turbofish = type_generics.as_turbofish();

    // get await if it is an async function
    let maybe_await = match old_fn.sig.asyncness {
        Some(..) => quote! {.await},
        None => quote! {},
    };

    let start_ident = get_random_name();

    let tag = old_name.to_string(); //TODO: allow for optional msg arg
    let tag = tag.as_str();

    // create body of new function which calls old function and converts error
    let new_fn_code_tokens: TokenStream = quote! {
        {
            let #start_ident = std::time::Instant::now();
            let output = #hidden_name #turbofish (#just_args) #maybe_await;
            time_it::action(Some(#tag), #start_ident.elapsed());
            output
        }
    }
    .into();

    let new_fn_code = parse_macro_input!(new_fn_code_tokens as syn::Block);

    new_fn.block = Box::new(new_fn_code);

    // write out the two functions
    let new_code: TokenStream = quote! {
        #new_fn
        #[inline(always)]
        #old_fn
    }
    .into();

    new_code
}

/// generates a random identifier to be used as the start instant name to avoid overwritting any other vars
fn get_random_name() -> Ident {
    let random_name = uuid::Uuid::new_v4().to_string().replace("-", "");

    Ident::new(
        &format!("start_{}", random_name),
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
