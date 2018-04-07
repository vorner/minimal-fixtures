#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{ArgCaptured, FnArg, Ident, Item, ItemFn, Type, TypeReference};

// TODO: Support for the question mark in tests, once it exists

fn wrap_call<'a, I: Iterator<Item = &'a FnArg>>(mut args: I, idx: usize, inner: Tokens) -> Tokens {
    let arg = args.next();
    match arg {
        None => inner,
        Some(&FnArg::Captured(ArgCaptured { ref ty, .. })) => {
            let rest = wrap_call(args, idx + 1, inner);
            let var_name = Ident::from(format!("__{}", idx));
            let raw_type = match ty {
                &Type::Reference(TypeReference { ref elem, .. }) => &elem,
                ty => ty,
            };
            quote!{
                for #var_name in <#raw_type as ::minimal_fixtures::Fixture>::values() {
                    #rest
                }
            }
        },
        _ => unimplemented!(),
    }
}

fn do_calls(fun: &ItemFn) -> Tokens {
    let params_raw = fun.decl.inputs
        .iter()
        .enumerate()
        .map(|(num, arg)| {
            let name = Ident::from(format!("__{}", num));
            match arg {
                &FnArg::Captured(ArgCaptured {
                    ty: Type::Reference(TypeReference {
                        mutability: Some(_),
                        ..
                    }),
                .. }) => quote!(&mut #name.clone()),
                &FnArg::Captured(ArgCaptured {
                    ty: Type::Reference(TypeReference {
                        mutability: None,
                        ..
                    }), ..
                }) => quote!(&#name),
                _ => quote!(#name.clone()),
            }
        });
    let comma = quote!(,);
    let mut params = Tokens::new();
    params.append_separated(params_raw, comma);
    let call = quote! {
        __internal(#params);
    };
    wrap_call(fun.decl.inputs.iter(), 0, call)
}

fn rename(fun: ItemFn) -> (Ident, ItemFn) {
    let ident = fun.ident;
    let renamed = ItemFn {
        attrs: Vec::new(),
        ident: Ident::from("__internal"),
        .. fun
    };
    (ident, renamed)
}

#[proc_macro_attribute]
pub fn minimal_fixture(_metadata: TokenStream, function: TokenStream) -> TokenStream {
    let function = match syn::parse(function).expect("Failed to parse tokens as function") {
        Item::Fn(function) => function,
        _ => panic!("#[minimal_fixture] can be applied only to #[test] functions"),
    };
    let (name, renamed) = rename(function);
    let calls = do_calls(&renamed);
    let result = quote! {
        #[test]
        fn #name() {
            #renamed
            #calls
        }
    };

    result.into()
}
