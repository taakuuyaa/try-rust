fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "str"
    };

    // numberの値は、{}です
    println!("The value of number is: {}", number);
}
