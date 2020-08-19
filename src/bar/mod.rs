//! This is a simple module at path crate::bar, with a definition of a type alias
pub type MyType = usize;

#[cfg(test)]
mod tests {
    #[test]
    pub fn it_works() {
        assert_eq!(2, 2);
    }
}
