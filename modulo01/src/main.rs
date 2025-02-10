use core::num;
// ? Lesson 04
use std::io; // library used for the user to use the data entry input

// create a function
fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

// ? End lesson 04

fn main() {
    // ? Lesson 01: variables
    // let name = "Henrique"; // variable declaration
    //let name: &str = "Henrique"; // variable declaration
    // ! error: variables are immutable in rust.
    // ! To change the value you need to add mut (changeable)
    //let mut name: &str = "Henrique";
    //name = "João"; 
    // let age = 42;
    // age += 1;
    //println!("Hello {}!", name); // {} defines the variable in the string

    // ? Lesson 02: Data types
    // ! To modify the type of integer we want to use, we have to specify it in the variable declaration
    // let x: i32 = 23; // standard integer: i32 (32bits)
    // let x: i64 = 23;
    //let x: u64 = 23;
    // let f: f32 = 6.7;
    //let f: f64 = 6.7;
    // let b: bool = false;
    //let b: bool = true;

    // ? Lesson 03: Control flow (if)
    // let number1 = 24;
    // let number2 = 42;
    // if number1 > number2{
    //     println!("{} > {}", number1, number2)
    // }else {
    //     println!("{} <= {}", number1, number2)
    // }

    // ? Lesson 04: Data entry and control flow for analysis
    // let mut number1 = String::new();
    // io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
    // let mut number2 = String::new();
    // io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

    // if convert_to_int(&number1) > convert_to_int(&number2){
    //     println!("O número {} é maior que {}", number1, number2);
    // }else{
    //     println!("O número {} é menor ou igual que {}", number1, number2);
    // }

    // ? Lesson 05: Sum of digits using while
    let mut soma = 0;
    let mut valor_entrada = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("Erro ao ler valor_entrada");
    
    let mut valor_i32 = convert_to_int(&valor_entrada);

    while valor_i32 != 0{
        let mut r = valor_i32 % 10;
        soma = soma + r;
        valor_i32 = valor_i32 / 10;
    }

    println!("O valor da soma dos dígitos é: {}", soma);

}
