fn main() {
    let sig_integer: i32 = -42;
    // there's i8, i16, i32, i64, i128

    let _unsigned_integer: u32 = 42;
    // there's u8, u16, u32, u64, u128 for unsigned

    let hi = "Hello World!";
    echo(hi);

    let _sig_integer_str: String = sig_integer.to_string();
    echo(&_sig_integer_str);

    // error: borrow of possibly-uninitialized variable: `x
    let x;
    // echo(x);
    // x = "oi";

    // However, doing this is completely fine:
    x = "hi";
    echo(x);

    let _ = foo();
    
    let res = foo();
    
    let x = foo();
    
    println!("{}", x.1);
    
    println!("{:?}", res);
    
    assert!(x.0.is_ok());
}

fn echo(input: &str) -> &str {
    println!("{}", input);
    input
}

fn foo() -> (Result<&'static str, &'static str>, &'static str) {
    (Ok("Ok"), "oi")
}