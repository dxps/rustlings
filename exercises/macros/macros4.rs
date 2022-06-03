// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    // first arm for no args
    () => {
        println!("Check out my macro!");
    };
    // second arm for one arg
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
