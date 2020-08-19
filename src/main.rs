#![allow(dead_code)]

mod bar;
mod errors;
mod lifetimes;
mod std_traits;
mod traits;

fn main() {
    println!("Hello, world!");
}

//
#[cfg(feature = "serialization")]
mod serde {

    #[cfg(test)]
    mod tests {
        #[test]
        fn serialize_it() {}
    }
}
