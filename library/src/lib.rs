use std::fmt::format;
use num::One;
use num_bigint::BigUint;
use crate::utils::hash_str;

pub mod utils;




#[derive(Debug)]
pub struct NICP {
    pub alpha: BigUint,
    pub beta: BigUint,
    pub modulus: BigUint,
    pub order: BigUint
}




pub fn gen_challenge(
    y_one: &BigUint,
    y_two: &BigUint,
    r_one: &BigUint,
    r_two: &BigUint
) -> BigUint {
    let to_hash_str = format!("{}{}{}{}",y_one,y_two,r_one,r_two);
    let hash__ = hash_str(&to_hash_str.as_str());

    BigUint::from_bytes_be(&hex::decode(hash__).unwrap())
}


pub fn solve_challenge(
    rand_prover_k: &BigUint,
    secret_from_prover: &BigUint,
    rand_verifier_c: &BigUint,
    modulus: &BigUint
) -> BigUint {
    let c_mul_x = rand_verifier_c * secret_from_prover;

    if *rand_prover_k > c_mul_x {
        (rand_prover_k - c_mul_x).modpow(&BigUint::one(), modulus)
    } else {
        modulus - (c_mul_x - rand_prover_k).modpow(&BigUint::one(), modulus)
    }
}


pub fn verify_challenge(
    alpha: &BigUint,
    beta: &BigUint,
    solution: &BigUint,
    challenge_in: &BigUint,
    y_one: &BigUint,
    y_two: &BigUint
) -> bool {
    // let gen_r_one
    // let gen_r_2
    // let gen_challenge


     // return condition
}