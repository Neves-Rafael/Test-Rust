// fn main() {
//     println!("Hello, world!");
// }

// const MINUTES_IN_HOUR: u32 = 60;
// const SECONDS_IN_MINUTES: u32 = 60;
// const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTES * MINUTES_IN_HOUR;
// fn main(){
//     let total = 30;
//     let total_em_segundos = total * SECONDS_IN_HOUR;

//     print!("Trabalhou { } segundos", total_em_segundos)
// }

// use std::io;

// fn main(){
//     let mut s = String::new();
//     println!("Digite um texto");
//     io::stdin()
//         .read_line(&mut s)
//         .expect("Error reading console");

//     println!("Você digitou {}",s);
//     // println!("Quantidade de letras {}", s.trim().len())
//     //.len() pega o tamanho em bits
//     //Para pegar o tamanho para cada posição
//     println!("Quantidade de letras {}", s.trim().chars().count())
// }

use std::io;

fn main() {
    println!("{:-^40}", "Calculadora");
    let mut s = String::new();
    let banner = "Digite uma sequencia de números\n\
    separados por vírgula\n\
    exemplo: 1,2,3,4,5";

    println!("{banner}");

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    let nums: Vec<i32> = s
        .split(",")
        .map(|c| c.trim().parse().expect("Error"))
        .collect();

    let result: i32 = nums.iter().sum();

    println!("O total é {}", result);
    println!("{}", "-".repeat(40));
}
