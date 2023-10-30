fn square(var:i32) -> i32 {
    var * var
}


fn main() {
    let var = 10;
    println!("{} ^ 2 : {}", var, square(var));
}
