extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe o palpite!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Digite o seu palpite: ");

        let mut palpite = String::new();
        /*
         * Outra possibilidade seria:
         * sdt::io:stdin() que seria a forma completa se não usassemos
         * std::io no escopo global
         */
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a entrada!"); // Tratar as potenciais falhas
        println!("Você disse: {}", palpite);

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
