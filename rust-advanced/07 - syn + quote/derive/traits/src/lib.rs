pub use macros::TypeInfo as TypeInfo;

pub trait TypeInfo {
    fn get_info() -> (usize, usize);
}
impl TypeInfo for u32 {
    fn get_info() -> (usize, usize) {
        (4, 4)
    }
}
