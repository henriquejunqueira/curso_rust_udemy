fn main() {
    // let name = "Henrique"; // variable declaration
    //let name: &str = "Henrique"; // variable declaration
    // ! error: variables are immutable in rust.
    // ! To change the value you need to add mut (changeable)
    let mut name: &str = "Henrique";
    name = "João"; 
    // let age = 42;
    // age += 1;
    println!("Hello {}!", name); // {} defines the variable in the string
}
