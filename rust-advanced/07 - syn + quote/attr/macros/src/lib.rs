use proc_macro2::{TokenStream, TokenTree, Group, Ident};
use quote::{ToTokens, TokenStreamExt};
use syn::ext::IdentExt;
use syn::parse::{Parse, Parser};
use syn::{Lit, LitStr, Meta, MetaList, MetaNameValue};
use syn::{punctuated::Punctuated, Token, Expr, ExprLit};

#[derive(Debug, Clone)]
struct ListOfIdents {
    idents: Vec<Ident>
}
impl ListOfIdents {
    fn empty() -> Self {
        ListOfIdents { idents: Vec::new() }
    }
}
impl Parse for ListOfIdents {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut idents = Vec::new();
        while !(input.is_empty() || input.peek(Token![,])) {
            idents.push(input.call(Ident::parse_any)?);
        }
        Ok(Self {
            idents
        })
    }
}
#[derive(Debug)]
enum Action {
    Payload(LitStr),
    Replace(Ident, ListOfIdents),
}
fn evaluate_payload(expr: Expr) -> Action {
    let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. } ) = expr else {
        panic!("Expected string literal for payload!");
    };
    Action::Payload(lit.clone())
}
fn evaluate_replace(tokens: TokenStream) -> Action {
    let maybe =
        Punctuated::<ListOfIdents, Token![,]>::parse_terminated.parse2(tokens);
    let Ok(list) = maybe else {
        panic!("Error: {}", maybe.err().unwrap());
    };
    assert!(list.len() > 0, "Expected at least one identifier.");
    assert!(list.len() <= 2, "Expected at most two identifiers.");
    let src = list[0].clone();
    if src.idents.len() != 1 {
        panic!("`replace()` requires one identifier to replace.");
    }
    if list.len() == 2 {
        Action::Replace(src.idents[0].clone(), list[1].clone())
    } else {
        Action::Replace(src.idents[0].clone(), ListOfIdents::empty())
    }
}
fn evaluate_attribute(attribute: Meta) -> Action {
    match attribute {
        Meta::NameValue(MetaNameValue { path, eq_token: _, value }) => {
            let ident = path.require_ident().unwrap_or_else(|e|{ panic!("{e}") });
            match ident.to_string().as_str() {
                "payload" => evaluate_payload(value),
                s => panic!("Unknown key `{s}`."),
            }
        }
        Meta::List(MetaList { path, delimiter: _, tokens }) => {
            let command = path.require_ident().unwrap_or_else(|e|{ panic!("{e}") });
            match command.to_string().as_str() {
                "replace" => evaluate_replace(tokens),
                s => panic!("Unknown command `{s}`."),
            }
        }
        m => todo!("{m:?}"),
    }
}

fn commit_actions(input: TokenStream, actions: &[Action]) -> TokenStream {
    let mut output = TokenStream::new();
    for tkn in input {
        match &tkn {
            TokenTree::Ident(ident) => {
                let mut possible = Vec::new();
                for a in actions {
                    if let Action::Replace(s, _) = a {
                        if s == ident { possible.push(a); }
                    }
                }
                if possible.is_empty() {
                    output.append(tkn);
                } else {
                    assert!(possible.len() == 1, "Multiple replacements are not supported.");
                    let Action::Replace(_, ListOfIdents { idents }) = possible[0] else {
                        unreachable!()
                    };
                    for id in idents {
                        output.append(id.clone());
                    }
                }
            } 
            TokenTree::Literal(lit) => {
                let Ok(_) = syn::parse2::<LitStr>(lit.into_token_stream()) else {
                    output.append(tkn);
                    continue;
                };
                let mut possible = Vec::new();
                for a in actions {
                    if matches!(a, Action::Payload(_)) {
                        possible.push(a);
                    }
                }
                assert!(possible.len() <= 1, "Multiple payloads are not supported.");
                if let Some(Action::Payload(payload)) = possible.get(0) {
                    output.append(payload.token());
                } else {
                    output.append(tkn);
                }
            }
            TokenTree::Punct(_) => output.append(tkn), 
            TokenTree::Group(group) => {
                let delim = group.delimiter();
                let elems = group.stream();
                let gr_out = commit_actions(elems, actions);
                let new = Group::new(delim, gr_out);
                output.append(new);
            }
        }
    }
    output
}
fn parse_actions(attr: TokenStream) -> Vec<Action> {
    let attr_list =
        Punctuated::<Meta, Token![,]>::parse_terminated.parse2(attr);
    let Ok(attributes) = attr_list else {
        panic!("Could not parse attributes!")
    };
    let mut actions = Vec::with_capacity(attributes.len());
    for attr in attributes {
        let action = evaluate_attribute(attr);
        actions.push(action);
    }
    actions
}
#[proc_macro_attribute]
pub fn inject(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let actions = parse_actions(attr.into());
    let output = commit_actions(item.into(), &actions);
    output.into()
}
