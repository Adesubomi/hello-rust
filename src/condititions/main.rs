fn main() {
    let mut x:u64 = 45;
    let f = 6.7;
    let b:bool = false;
    println!("The value of x is {}", x);
    println!("We have a float value f of :: {}", f);
    println!("We also have a bool whose value is :: {}", b);

    x = 12;
    println!("The NEW value of x is {}", x);

    if x > 15 {
        println!("    - - The value of x is greater than 15");
    } else if !b {
        println!("    - - We're not Rusty!");
    }
}
