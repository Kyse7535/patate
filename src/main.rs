mod hash;

// use std::net::TcpStream;

// fn main() {
//     println!("Tentative de connexion au serveur...");
//     match TcpStream::connect("127.0.0.1:7878") {
//         Ok(_) => {
//             println!("Connexion au serveur réussie !");
//         }
//         Err(e) => {
//             println!("La connexion au serveur a échoué : {}", e);
//         }
//     }
// }

extern crate md5;
//use hex_literal::hex;

struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}

fn the_hash_cash() {
//let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
// create a Md5 hasher instance
let digest = md5::compute(b"000000000000034Chello");

// process input message
println!("{:x}", digest);
}

fn bit_a_zero_in_a_char(mychar: Option<char>) -> u32 {
    match mychar {
        Some('0') => 4,
        Some('1') => 3,
        Some('2') => 2,
        Some('3') => 2,
        Some('4') => 1,
        Some('5') => 1,
        Some('6') => 1,
        Some('7') => 1,
        _ => 0
    }
}

fn count_bit_a_zero(msg: &str) -> u32 {
    match msg.len() {
        0 => 0,
        _ => {
            let mut nbre = 0;
            let mut mychar = msg.chars();
            let mut i = mychar.next();
            while i == Some('0') {
                nbre += 4;
                i = mychar.next();
            }
            nbre += bit_a_zero_in_a_char(i);
            nbre
        }
    }
}

fn main() {
    // let list = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8","9", "A", "B", "C", "D", "E", "F"];

    println!("{}", bit_a_zero_in_a_char(Some('5')));
    let a = count_bit_a_zero("0001745D9BDF8E5D3C7872AC9DBB2C3");
    println!("{}", a);

}