use quote::quote;
use proc_macro::{TokenStream};
use proc_macro2::{TokenStream as TS, TokenTree as TT};
use proc_macro2::{Ident, Span};
use std::collections::HashSet;

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
