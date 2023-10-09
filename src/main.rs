fn main() {
    let mut arr = String::from("hello world");
    println!("{}", arr);
    arr.push('!');
    println!("{}", arr);
    arr.pop();
    println!("{}", arr);
    arr.pop();
    println!("{}", arr);
}
