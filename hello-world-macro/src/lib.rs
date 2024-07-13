use quote::quote;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{TokenStream as TS, TokenTree as TT};
use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, DeriveInput};
use std::collections::HashSet;

#[proc_macro_derive(Hello)]
pub fn hello(input: TokenStream) -> TokenStream {
    println!("in hello: {input:?}");
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let hw = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello macro world");
            }
        }
    };
    hw.into()
}

#[proc_macro_derive(HelloAlt)]
pub fn hello_alt(input: TokenStream) -> TokenStream {
    fn ident_name(tt: TokenTree) -> String {
        match tt {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("no ident")
        }
    }
    let name = ident_name(input.into_iter().nth(1).unwrap());

    format!("
impl {name} {{
  fn hello_world_alt(&self) {{
    println!(\"Hello manual macro world\");
  }}
}}").parse().unwrap()
}

#[proc_macro]
pub fn biscuit(input: TokenStream) -> TokenStream {
    let input: TS = input.into();
    println!("biscuit inputs: {input:?}");
    let idents: Vec<_> = input.into_iter().filter_map(|x| match x {
        TT::Ident(i) => Some(quote!{
            #i: u32
        }),
        _ => None
    }).collect();
    println!("biscuit idents: {idents:?}");
    let foo = Ident::new("foo", Span::call_site());
    let footyp = Ident::new("i64", Span::call_site());
    let out = quote! {
        #[derive(Debug, Default)]
        struct Biscuit {
            #foo : #footyp,
            #( #idents ,)*
        }
    };
    out.into()
}

const OK_FIELDS: &[(&'static str, &'static str)] = &[
    ("checksum", "u32"),
    ("global_seq_num", "u64"),
    ("tstamp", "u64"),
];

#[proc_macro]
pub fn ok_struct(input: TokenStream) -> TokenStream {
    let input: TS = input.into();
    let mut idents = input.into_iter().filter(|x| matches!(x, TT::Ident(_)));
    let name = idents.next().expect("ok_struct expects at least one ident for the struct name");
    let wanted: HashSet<_> = idents.map(|x| x.to_string()).collect();
    let fields: Vec<_> = OK_FIELDS.iter().enumerate().map(|(i, (f, t))| {
        let f = if wanted.contains(&f.to_string()) {
            f.to_string()
        } else {
            format!("padding_{i}")
        };
        let f = Ident::new(&f, Span::call_site());
        let t = Ident::new(t, Span::call_site());
        quote! { #f : #t }
    }).collect();
    let out = quote! {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default)]
        struct #name {
            #( #fields ,)*
        }
    };
    out.into()    
}
