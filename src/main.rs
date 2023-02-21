use  std::io;
fn main() {
    //println!("Hello, world!");
    println!("guess number");

    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("error");
    println!("{}",str)

}
