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

fn main() {
    the_hash_cash();
}