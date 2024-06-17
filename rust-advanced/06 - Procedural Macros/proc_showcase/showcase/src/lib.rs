
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn function_like(item: TokenStream) -> TokenStream {
    for tt in item.into_iter() {
        match tt {
            TokenTree::Ident(id) => println!("{:?}", id),
            TokenTree::Punct(pt) => println!("{:?}", pt),
            TokenTree::Literal(lit) => println!("{:?}", lit),
            TokenTree::Group(grp) => {
                println!("{:?}", grp.delimiter());
                for gtt in grp.stream() {
                    println!("  {:?}", gtt);
                }
            }
        }
    }
    "".parse().unwrap()
}

#[proc_macro]
pub fn create_struct(values: TokenStream) -> TokenStream {
    let mut tokens = values.into_iter().peekable();
    let Some(TokenTree::Ident(name)) = tokens.peek() else {
        panic!("Expected identifier, found {:?}", tokens.peek());
    };
    let mut result = format!("struct {name} {{");
    tokens.next();
    while let Some(TokenTree::Ident(field)) = tokens.next() {
        let Some(TokenTree::Ident(typ)) = tokens.peek() else {
            panic!("Expected type, found {:?}", tokens.peek());
        };
        result.push_str(&format!("{field}: {typ},"));
        tokens.next();
    }

    result.push('}');
    result.parse().unwrap()
}

#[proc_macro_derive(DeriveTrait)]
pub fn trait_derive(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();
    tokens.next(); // Skip `struct` or `enum`
    let Some(TokenTree::Ident(name)) = tokens.peek() else {
        panic!("Expected struct with name, got {:?}", tokens.peek());
    };
    let name = name.to_string();
    tokens.next();
    format!("
    impl OurTrait for {name} {{
        fn greet(&self) {{
            println!(\"Hello {name}!\");
        }}
    }}
    ").parse().unwrap()
}

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    for tt in item.into_iter() {
        match tt {
            TokenTree::Ident(id) => println!("{:?}", id),
            TokenTree::Punct(pt) => println!("{:?}", pt),
            TokenTree::Literal(lit) => println!("{:?}", lit),
            TokenTree::Group(grp) => {
                println!("{:?}", grp.delimiter());
                for gtt in grp.stream() {
                    println!("  {:?}", gtt);
                }
            }
        }
    }
    "".parse().unwrap()
}
