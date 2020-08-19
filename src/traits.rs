/// a simple trait, with no generics
trait Simple {
    fn as_simple(&self) -> usize;
}

struct Bar;
impl Simple for Bar {
    fn as_simple(&self) -> usize {
        42usize
    }
}

fn use_simple() -> usize {
    let bar = Bar;
    bar.as_simple()
}

// a trait with a generic type argument, that determines the return type
trait AsFoo<Foo> {
    fn as_foo(&self) -> Foo;
}

impl AsFoo<String> for Bar {
    fn as_foo(&self) -> String {
        "Foo".to_string()
    }
}

impl AsFoo<usize> for Bar {
    fn as_foo(&self) -> usize {
        42usize // or 42
    }
}

fn use_bar() {
    let bar = Bar;
    let str: String = bar.as_foo();
    let str: usize = bar.as_foo();
}

// Associated traits
trait AssociatedFoo {
    type Foo;

    fn associated_foo(&self) -> Self::Foo;
}

struct BarAssociated;
impl AssociatedFoo for BarAssociated {
    type Foo = String;

    fn associated_foo(&self) -> String {
        "Foo".to_string()
    }
}

// uncommenting will error! associated types are unique for the implemented trait
// impl AssociatedFoo for BarAssociated {
//     type Foo = String;

//     fn as_foo() -> String {
//         "Foo".to_string()
//     }
// }

fn use_bar_associated() {
    let bar = BarAssociated;
    let str: String = bar.associated_foo();
    // let str: usize = bar.associated_foo();
}

trait AssociatedGenericFoo<Generic> {
    type Foo;

    fn both_foo(&self) -> Self::Foo;
}

struct BarBoth;
struct A;
struct B;

impl AssociatedGenericFoo<A> for BarBoth {
    type Foo = String;

    // you can use Self::Foo
    fn both_foo(&self) -> Self::Foo {
        "foo".to_string()
    }
}

impl AssociatedGenericFoo<B> for BarBoth {
    type Foo = usize;

    // or the actual type
    fn both_foo(&self) -> usize {
        42usize
    }
}

fn use_both() {
    let bar = BarBoth;
    let str: String = AssociatedGenericFoo::<A>::both_foo(&bar);
    let str: usize = AssociatedGenericFoo::<B>::both_foo(&bar);
}
