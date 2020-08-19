fn string_str() {
    // this is a reference.  it is a pointer that points to the type `str`, a primitive
    // because the str is stored in the executable, the reference is valid for the entire program lifetime
    let static_reference = "foo";

    // this is an owned value.  it is stored on the stack
    let owned = "foo".to_string();

    // if we uncomment this, the owned value is moved into the function call
    // we see a compile error below
    // use_owned(owned);

    // this is a reference, valid for the lifetime of `owned`
    let borrowed = owned.as_str();

    use_borrowed(borrowed);

    // this is an unsafe function call.
    // in this case, the `as_bytes_mut` function requires that we enforce some additional constraints about how we call it
    // in this case, it's because the string needs to always be valid utf-8 - it's a language feature of Rust.
    let mut mutable_string = "foo".to_string();

    unsafe {
        // owned isn't marked as mut! so we can't get a mutable reference
        // owned.as_bytes_mut()[0] = b'm';
        mutable_string.as_bytes_mut()[0] = b'm';
    }

    // prints 'moo'
    println!("{}", mutable_string.as_str());
}

fn use_owned(string: String) {}

fn use_borrowed(string: &str) {
    println!("{}", string);
}

fn mutable_borrows() {
    // mut refers to the local variable binding, not a reference
    let mut data = vec![1, 2, 3];

    // this is a reference, it can be shared with other functions
    let data_mut = &mut data;
    do_something(data_mut);

    // but if data is borrowed mutably, we can't read from it
    // > cannot borrow `data` as immutable because it is also borrowed as mutable
    // > immutable borrow occurs here

    // read_something(&data);

    data_mut[1] = 4;
}

fn do_something(data: &mut Vec<usize>) {
    // ...
}

fn read_something(data: &Vec<usize>) {
    // ...
}

struct BorrowedData<'lifetime, Value> {
    pub borrow: &'lifetime Value,
}

impl<'lifetime, Value> AsRef<Value> for BorrowedData<'lifetime, Value> {
    fn as_ref(&self) -> &Value {
        self.borrow
    }
}

struct OwnedData<Value> {
    pub owned: Value,
}
impl<Value> OwnedData<Value> {
    // consumes self, and returns value
    pub fn into_value(self) -> Value {
        self.owned
    }
}
fn struct_lifetime() {
    // define some data
    let data = vec![1, 2, 3];

    let borrow = BorrowedData { borrow: &data };
    let another_borrow = borrow.as_ref();

    let owned = vec![1, 2, 3];
    // this is shorthand if a local variable and the type definition have the same name
    let owned_struct = OwnedData { owned };
    // we can borrow against owned_struct at this point

    // we consume the struct, and return the value
    let extracted = owned_struct.into_value();
}
