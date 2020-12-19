use crate::entities::traits::properties::Printable;

#[cfg(not(target_arch = "wasm32"))]
pub fn print_str(s: &'static str) {
    println!("{:?}", s);
}

#[cfg(target_arch = "wasm32")]
pub fn print_str(s: &'static str) {
    print_string(s.to_owned());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn print_data(data: impl Printable) {
    print_string(data.as_printable());
}

#[cfg(target_arch = "wasm32")]
pub fn print_data(data: impl Printable) {
    print_string(data.as_printable());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn print_string(s: String) {
    println!("{:?}", s);
}

#[cfg(target_arch = "wasm32")]
pub fn print_string(s: String) {
    use web_sys::console;
    console::log_1(&s.into());
}
