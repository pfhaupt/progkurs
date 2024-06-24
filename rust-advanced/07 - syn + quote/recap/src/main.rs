#![allow(unused)]
use fn_like::print_type_info;
struct Custom {
    field: u32,
    other: Option<&'static str>
}
fn main() {
    print_type_info!(u32);
    print_type_info!(String);
    print_type_info!(Custom);
    print_type_info!((&str, u8, bool));
}
