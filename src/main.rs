#![allow(dead_code)]

mod bar;
mod errors;
mod lifetimes;
mod std_traits;
mod traits;

// this is the entrypoint for your executable.
// if you do `cargo run`, hello world will be printed
// you can also return Result<(), Error>,
// and Rust will print an error message if Err is returned
fn main() {
    println!("Hello, world!");
}

// unit tests can be placed anywhere.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2, 1 + 1)
    }
}

// this is conditional compilation.  users of your library/executable can toggle on and off code.
#[cfg(feature = "serialization")]
mod serde {
    #[cfg(test)]
    mod tests {
        #[test]
        fn serialize_it() {}
    }
}
