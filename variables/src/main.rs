fn main() {

    // mut example.
    let mut x = 5;
    println!("The value of x = {}", x);
    x = 6;
    println!("The value of x = {}", x);

    // Shadowing example
    let y = 5;
    let y = y+1;
    let y=y*2;
    println!("y = {}", y);

    // This is perfectly fine.
    let spaces = " ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // This gives a compile-time error.
    // let mut spaces = " ";
    // spaces = spaces.len();

    // PAGE 52
}
