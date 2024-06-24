#![allow(unused)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, ItemFn, ItemStruct, Ident};

fn gather_fields(data: &Data) -> Vec<&Ident> {
    let Data::Struct(strukt) = data else {
        panic!("Only structs are supported for now.");
    };
    let mut logged = Vec::new();
    for field in &strukt.fields {
        if let Some(attr) = field.attrs.get(0) {
            let path = attr.path();
            let Ok(ident) = path.require_ident() else {
                panic!("Only `log` is a valid attribute")
            };
            let name = ident.to_string();
            assert!(name == "log", "Only `log` is a valid attribute.");
            let Some(ident) = field.ident.as_ref() else {
                panic!("Tupled structs are not supported yet.")
            };
            logged.push(ident);
        }
    }
    logged
}

#[proc_macro_derive(TypeInfo, attributes(log))]
pub fn derive_type_info(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs, vis, ident, generics, data } = parse_macro_input!(input);
    let logged = gather_fields(&data);
    let typ = quote! { #ident #generics };
    quote! {
        impl #generics TypeInfo for #typ {
            fn get_info() -> (usize, usize) {
                #(println!("Logging field `{}`", stringify!(#logged)););*
                ( std::mem::size_of::<#typ>(), std::mem::align_of::<#typ>() )
            }
        }
    }.into()
}
/*
    let DeriveInput { attrs, vis, ident, generics, data } = parse_macro_input!(input);
    let typ = quote! { #ident #generics };
    let Data::Struct(strukt) = data else { unimplemented!() };
    let mut logged = Vec::new();
    for field in &strukt.fields {
        if let Some(attr) = field.attrs.get(0) {
            let path = attr.path();
            let ident = path.require_ident().unwrap_or_else(|e|panic!("{e}"));
            let name = ident.to_string();
            assert!(name == "log", "Only `log` is a valid attribute.");
            logged.push(field.ident.as_ref().expect("Tupled structs are not supported yet."));
        }
    }
    quote! {
        impl #generics TypeInfo for #typ {
            fn get_info() -> (usize, usize) {
                #(println!("Logging field `{}`", stringify!(#logged)););*
                ( std::mem::size_of::<#typ>(), std::mem::align_of::<#typ>() )
            }
        }
    }.into()
 */
// #[proc_macro_derive(TypeInfo)]
// pub fn derive_type_info(input: TokenStream) -> TokenStream {
//     let DeriveInput { ident, generics, ..} = parse_macro_input!(input);
//     let typ = quote! { #ident #generics };
//     quote! {
//         impl #generics TypeInfo for #typ {
//             fn get_info() -> (usize, usize) {
//                 ( std::mem::size_of::<#typ>(), std::mem::align_of::<#typ>() )
//             }
//         }
//     }.into()
// }