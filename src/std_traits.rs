#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Foo {
    value: String,
}

#[cfg(test)]
mod tests {
    use super::Foo;

    #[test]
    fn test_eq() {
        let foo1 = Foo {
            value: "a".to_string(),
        };
        let foo2 = Foo {
            value: "a".to_string(),
        };
        assert_eq!(foo1, foo2);
    }

    #[test]
    fn test_ord() {
        let a = Foo {
            value: "a".to_string(),
        };
        let b = Foo {
            value: "b".to_string(),
        };
        assert!(a < b);
    }
}
