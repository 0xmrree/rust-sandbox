fn main() {    // Create an Option with Some value
    let some_value: Option<i32> = None;
    
    // Unwrap the Option to get the value
    let unwrapped = some_value.unwrap();
    println!("Unwrapped value: {}", unwrapped);
}
