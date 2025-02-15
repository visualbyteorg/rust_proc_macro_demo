use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes {
    #[hello_macro(greeting = "Large size!")]
    size: u32,
}
fn main() {
    Pancakes::hello_macro();
}