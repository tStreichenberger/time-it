extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use quote::ToTokens;
use syn::{
    Result as SynResult,
    Stmt,
    Block,
    parse::{
        Parse,
        ParseStream
    },
};

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


// #[proc_macro]
// pub fn msg(inp: TokenStream) -> TokenStream {inp}

#[proc_macro_attribute]
pub fn msg(_: TokenStream, inp: TokenStream) -> TokenStream {inp}

const MSG_NAME: &str = "time_it_msg";

#[proc_macro]
pub fn time_it_2(to_time: TokenStream) -> TokenStream {
    // let mut input = syn::parse_macro_input!(to_time as TimeItInput);
    // let attrs = input.attrs;
    // panic!("{:#?}", attrs);
    let input = to_time;

    let trees: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    panic!("{:#?}", trees);

    // let mut attrs = syn::parse_macro_input!(to_time as TimeItMsg);

    // let stmts = input.stmts;

    // let stmts = syn::Block::parse_within(to_time.into()).unwrap();
    // let stmts = syn::parse_macro_input!(to_time as Vec<syn::Stmt>);
    // let to_time = syn::Item::Verbatim();

    // let to_time = stmts.into_iter().map(|stmt| syn::Item::Verbatim(stmt.into_token_stream())).collect::<Vec<_>>();

    let mut msg = syn::Lit::Str(syn::LitStr::new("Operation", proc_macro::Span::call_site().into()));

    // for stmt in input.stmts.iter_mut() {
    //     match stmt {
    //         Stmt::Local(local) => {
    //             let mut found_msg = false;
    //             for attr in local.attrs.iter() {
    //                 // search for time msg macro
    //                 let attr_name = attr
    //                     .clone()
    //                     .path
    //                     .segments
    //                     .iter()
    //                     .last()
    //                     .expect("malformed attribute")
    //                     .ident
    //                     .to_string();
    //                 if attr_name.as_str() != "msg" {
    //                     continue;
    //                 }
    //                 // TODO: use index
    //                 found_msg = true;
                    
    //                 let meta = attr.parse_meta().expect("Malformed Attribute");
    //                 let list_macro = match meta {
    //                     syn::Meta::List(l) => l,
    //                     _ => panic!("Expected derive opts to have arguments"),
    //                 };
    //                 for arg in list_macro.nested {
    //                     match arg {
    //                         syn::NestedMeta::Meta(_) => panic!("time_it::msg must me literal"),
    //                         syn::NestedMeta::Lit(lit) => {msg = lit; break},
    //                     }
    //                 }
    //             }
    //             if found_msg {
    //                 local.attrs = vec![];
    //             }
    //         }
    //         _ => continue
    //     }
    // }

    quote! {
        // let start = std::time::Instant::now();
        // #input
        // println!("{} took {}ms", #msg, start.elapsed().as_millis())
    }
    .into()
}


struct TimeItInput {
    stmts: Vec<Stmt>,
    attrs: Vec<syn::Attribute>
}

impl Parse for TimeItInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let stmts = input.call(Block::parse_within)?;
        let attrs = input.call(syn::Attribute::parse_outer)?;

        Ok(TimeItInput { stmts, attrs })
    }
}


impl ToTokens for TimeItInput {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        self.stmts.iter().for_each(|stmt| {
            stmt.to_tokens(tokens)
        })
    }
}


// struct TimeItMsg {
//     attrs: Vec<syn::Attribute>
// }

// impl Parse for TimeItMsg {
//     fn parse(input: ParseStream) -> SynResult<Self> {
//     }
// }