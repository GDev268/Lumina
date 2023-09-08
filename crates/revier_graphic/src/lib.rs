pub mod pipeline;
pub mod renderer;
pub mod shader;

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        unsafe {
            let b: $base = std::mem::zeroed();
            (std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize).try_into().unwrap()
        }
    }};
}
