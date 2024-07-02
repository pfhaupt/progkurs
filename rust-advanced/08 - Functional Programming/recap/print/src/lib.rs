use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, Expr, ExprBlock, ItemFn, Local, LocalInit, Pat, Stmt};

fn replace_in_block(block: &Block) -> Block {
    let mut new_stmts = vec![];
    for s in &block.stmts {
        match s {
            Stmt::Local(Local { pat: Pat::Ident(var), init: Some(LocalInit { expr, .. }), .. }) => {
                // let <pat> = <expr>;
                // <pat> is a normal identifier like `a` or `b`
                let log = quote! {
                    let #var = {
                        let _e = #expr;
                        let _print_green = |s: &str| -> String { format!("\x1b[92m{s}\x1b[0m") };
                        let _print_yellow = |s: &str| -> String { format!("\x1b[93m{s}\x1b[0m") };
                        println!("Setting {} to {}",
                            _print_green(stringify!(#var)),
                            _print_yellow(&format!("{} = {:?}", stringify!(#expr), _e))
                        );
                        _e
                    };
                };
                let stmt = syn::parse2::<Stmt>(log).unwrap();
                new_stmts.push(stmt);
            }
            Stmt::Local(Local { pat: Pat::Ident(var), init: None, .. }) => {
                // let <pat>;
                // <pat> is a normal identifier like `a` or `b`
                new_stmts.push(s.clone());
                let log = quote! {
                    {
                        let _print_green = |s: &str| -> String { format!("\x1b[92m{s}\x1b[0m") };
                        println!("Declaring {}", _print_green(stringify!(#var)));
                    }
                };
                let stmt = syn::parse2::<Stmt>(log).unwrap();
                new_stmts.push(stmt);
            }
            Stmt::Expr(Expr::Block(ExprBlock { attrs: a, label: l, block: inner, }), semi) => {
                let new_inner = replace_in_block(inner);
                new_stmts.push(Stmt::Expr(Expr::Block(ExprBlock { attrs: a.clone(), label: l.clone(), block: new_inner}), *semi));
            }
            _ => new_stmts.push(s.clone()),
        }
    }
    Block { brace_token: block.brace_token, stmts: new_stmts }
}

#[proc_macro_attribute]
pub fn log_variable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block
    } = parse_macro_input!(item);
    let new_block = replace_in_block(&block);
    quote! {
        #(#attrs)*
        #vis #sig #new_block
    }.into()
}

