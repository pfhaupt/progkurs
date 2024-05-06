use proc_macro::TokenStream;
use quote::{format_ident, quote};
use rand::seq::SliceRandom;

static mut USE_REF: bool = true;
static mut LOOP_COUNT: usize = 0;
static mut STRUCT_COUNT: usize = 0;

#[proc_macro]
pub fn use_ref(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();
    assert!(tokens.len() == 1);
    let proc_macro::TokenTree::Ident(ident) = &tokens[0] else {
        panic!();
    };
    let ident = ident.to_string();
    let use_ref = str::parse::<bool>(&ident).unwrap();
    unsafe {
        USE_REF = use_ref;
    }
    quote!().into()
}

#[proc_macro]
pub fn struct_count(input: TokenStream) -> TokenStream {
    let amt = parse_amt(input);
    unsafe {
        STRUCT_COUNT = amt;
    }
    quote!().into()
}
#[proc_macro]
pub fn loop_count(input: TokenStream) -> TokenStream {
    let amt = parse_amt(input);
    unsafe {
        LOOP_COUNT = amt;
    }
    quote!().into()
}

fn parse_amt(input: TokenStream) -> usize {
    let tokens: Vec<_> = input.into_iter().collect();
    assert!(tokens.len() == 1);
    let proc_macro::TokenTree::Literal(lit) = &tokens[0] else {
        panic!();
    };
    let lit = lit.to_string();
    let amt = str::parse::<usize>(&lit).unwrap();
    amt
}

#[proc_macro]
pub fn gen_flags(input: TokenStream) -> TokenStream {
    unsafe {
        println!("USE_REF: {USE_REF}");
    }
    let amt = parse_amt(input);
    let amt_range = 0..amt;
    /*
    Helper macro to let me showcase the power of references.
    Works by spamming the following code:
    #[derive(Clone)]
    struct Flags {
        [amt] random flags
    }
    impl Flags {
        fn parse(args: &std::env::Args) -> Self {
            Self {
                [amt] random initializers
            }
        }
    }
    or the clone-equivalent if USE_REF is false.
    */
    let fields: Vec<_> = amt_range.map(|i|{
        let name = format_ident!("f{i}");
        let t = ["f32", "i32", "String"].choose(&mut rand::thread_rng()).unwrap();
        let typ = format_ident!("{t}");
        (name, typ)
    }).collect();
    let f_names: Vec<_> = fields.iter().map(|(n, _)|n).collect();
    let f_types: Vec<_> = fields.iter().map(|(_, t)|t).collect();
    let mut output = quote!(
        impl std::fmt::Display for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Readonly CLI flags that you need everywhere")
            }
        }
        struct Flags {
            #(#f_names: #f_types,)*
        }
    );
    output.extend(quote!(
        impl Clone for Flags {
            fn clone(&self) -> Self {
                Self {
                    #(#f_names: self.#f_names.clone(),)*
                }
            }
        }
        impl Flags {
            fn parse(args: &std::env::Args) -> Self {
                Self {
                    #(#f_names: #f_types::default(),)*
                }
            }
        }
    ));
    output.into()
}

#[proc_macro]
pub fn gen_structs(input: TokenStream) -> TokenStream {
    assert!(input.to_string().is_empty());
    /*
    Helper macro to let me showcase the power of references.
    Works by spamming [amt] times the following code:

    #[allow(unused)]
    struct S[i]<'f> {
        flags: &'f Flags
    }
    impl<'f> S[i]<'f> {
        fn new(flags: &'f Flags) -> Self {
            Self {
                flags
            }
        }
    }

    or the clone-equivalent if USE_REF is false.
    */
    let mut output = quote!();
    for i in 0..(unsafe { STRUCT_COUNT }) {
        let s_name = format_ident!("S{i}");
        let f_name = format_ident!("f{i}");
        output.extend(if unsafe { USE_REF } {
            quote!(
            #[allow(unused)]
            struct #s_name<'f> {
                flags: &'f Flags
            }
            impl<'f> #s_name<'f> {
                fn new(flags: &'f Flags) -> Self {
                    Self {
                        flags
                    }
                }
            })
        } else {
            quote!(
            #[allow(unused)]
            struct #s_name {
                flags: Flags
            }
            impl #s_name {
                fn new(flags: &Flags) -> Self {
                    Self {
                        flags: flags.clone()
                    }
                }
            }
            fn #f_name(flags: &Flags) {
                let _ = #s_name::new(flags);
            })
        });
    }
    output.into()
}


#[proc_macro]
pub fn setup_structs(input: TokenStream) -> TokenStream {
    assert!(input.to_string().is_empty());
    /*
    Helper macro to let me showcase the power of references.
    Works by spamming [amt] times the following code:

    S[i]::new(&flags);

    or the clone-equivalent if USE_REF is false.
    */
    let mut output = quote!();
    let lc = unsafe { LOOP_COUNT };
    for i in 0..(unsafe { STRUCT_COUNT }) {
        let s_name = format_ident!("S{i}");
        let f_name = format_ident!("f{i}");
        output.extend(if unsafe { USE_REF } {
            quote!(
                for _ in 0..#lc {
                    #s_name::new(&flags);
                }
            )
        } else {
            quote!(
                for _ in 0..#lc {
                    #f_name(&flags);
                }
            )
        });
    }
    output.into()
}
