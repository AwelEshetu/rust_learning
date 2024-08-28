fn main() {
    let message = "Hello, world!";
    let weight=190.0;
    let kilos = weight / 2.2;
    println!("{} is {} kilos", message, kilos);
    let mut height = 72; // mutable variable
    height = height + 100; //shadowing changes the value of height
    print!("Height is now {}", height);
}
