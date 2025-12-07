// Testing variant name conflicts

enum MyEnum<T> {
    Some(T, T),
    None,
}

use std::option::Option::*;  // Brings Option's Some and None into scope
use MyEnum::*;                // Brings MyEnum's Some and None into scope

fn main() {
    // This should cause an ambiguity error
    let x = Some(5);
    println!("{:?}", x);
}
