extern crate proc_macro;
use proc_macro::TokenStream;
use itertools::Itertools; // for .join()
#[proc_macro]
pub fn print_type_info(input: TokenStream) -> TokenStream {
    let typ = input.into_iter().map(|t|t.to_string()).join("");
    format!("
    {{
        println!(\"Provided type: {typ}\");
        println!(\"Size: {{}} bytes\", std::mem::size_of::<{typ}>());
        println!(\"Alignment: {{}} bytes\", std::mem::align_of::<{typ}>());
        println!(\"\");
    }}
    ").parse().unwrap()
}
