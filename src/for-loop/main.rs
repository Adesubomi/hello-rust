fn main() {
    // let numbers = 30..51;

    let animals = vec!["Rabbit", "Dog", "Cat"];
    for (i, name) in animals.iter().enumerate() {
//        if i % 5 == 0 {
//            println!("The number is {}", i);
//        }
        println!("The animal name is {}: {}", i, name)
    }
}