use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, AttrStyle, Expr, FnArg, Ident, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn c_error(default_call: TokenStream, input: TokenStream) -> TokenStream {
    let default_call = parse_macro_input!(default_call as Expr);

    let mut function = parse_macro_input!(input as ItemFn);
    let new_name = format!("c_error_{}", function.sig.ident);
    let wrapper = function.clone();
    function.sig.ident = Ident::new(&new_name, Span::call_site());
    let wsig = wrapper.sig;
    let (outer, inner): (Vec<_>, Vec<_>) =
        wrapper
            .attrs
            .into_iter()
            .partition(|attr| match attr.style {
                AttrStyle::Outer => true,
                _ => false,
            });
    let args: Vec<_> = wsig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(r) => quote!(#r),
            FnArg::Typed(pat) => {
                let pat = &pat.pat;
                quote!(#pat)
            }
        })
        .collect();
    let constness = function.sig.constness;
    let asyncness = function.sig.asyncness;
    let unsafety = function.sig.unsafety;
    let abi = function.sig.abi;
    let ident = function.sig.ident;
    let generics = function.sig.generics;
    let where_clause = generics.where_clause.clone();
    let output = match function.sig.output {
        ReturnType::Type(_, ret_type) => quote!(anyhow::Result<#ret_type>),
        ReturnType::Default => quote!(anyhow::Result<()>),
    };
    let inputs = function.sig.inputs;
    let block = function.block;
    let out = quote! {
        #(#outer)*
        #wsig {
            #(#inner)*
            match #ident ( #(#args),* ) {
                Ok(v) => v,
                Err(e) => {
                    crate::error::set_last_error(e.into());
                    #default_call
                }
            }
        }
        #constness #asyncness #unsafety #abi fn #ident #generics (#inputs) -> #output #where_clause
            #block

    };
    out.into()
}
