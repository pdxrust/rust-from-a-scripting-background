fn and(x: bool,  y: bool) -> bool{
    x && y
}
fn another_and(x: bool,  y: bool) -> bool{
    return x && y;
}
fn main() {
    println!("{}", and(true, false));
    println!("{}", another_and(true, false));
}
