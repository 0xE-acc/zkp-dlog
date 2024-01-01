use num_bigint::{BigUint, ToBigUint};
use rand::Rng;
use primal::is_prime;
use num_traits::{One, ToPrimitive};

fn generate_large_prime(bits: usize) -> BigUint {
    // generate random number with desired number of bits
    let mut rng = rand::thread_rng();
    let mut candidate = rng.gen_range(0u64..(1u64 << bits)).to_biguint().unwrap();

    // use primality test to find next prime >= candidate
    let prime = loop {
        if is_prime(candidate.to_u64().unwrap()) {
            break candidate.clone();
        }
        candidate += 1u32;
    };
    prime
}

fn dlog_proof1(x: i32, g: i32, p: i32) -> i32 {
    // step one of proof
    let mut y = x.pow(g as u32);
    y = y % p;

    y
}

fn main() {
    let mut rng = rand::thread_rng();
    
    let rand_generator = rng.gen::<i32>();
    println!("Generator: {:?}", rand_generator);
    
    let secret_id = rng.gen::<i32>();
    println!("Secret ID: {:?}", secret_id);
    
    let large_prime = generate_large_prime(4);
    println!("Large Prime: {:?}", large_prime);

    // prover sends y = x^g(mod p)
    println!("Prover calculating y = rand_generator ^ secret_id");
    let _h = dlog_proof1(rand_generator, secret_id, large_prime.clone().to_u32().unwrap() as i32);

    // verifier sends back random bit 
    println!("Verifier sending random bit...");
    let verifier_random_bit = rng.gen_range(0..=1);

    // prover sends s = (rand_generator + verifier_random_bit * secret_id)(mod (large_prime - 1))
    println!("Prover sending s = (rand_generator + verifier_random_bit * secret_id)(mod (large_prime - 1))");
    let mut s = rand_generator + verifier_random_bit * secret_id;
    s = s % (large_prime.clone() - BigUint::one()).to_u32().unwrap() as i32;

    println!("Verifier computing rand_generator ^ s(mod large_prime)");
    let mut verifier_solution = rand_generator.to_biguint().unwrap().pow(s as u32) % &large_prime;
    verifier_solution = verifier_solution % &large_prime;
    
    println!("verifier_solution should equal h^verifier_random_bit(mod large_prime)");
    let compared_solution = verifier_solution.modpow(&(verifier_random_bit as u32).into(), &large_prime);
    println!("verifier_solution: {}, compared_solution: {}", verifier_solution, compared_solution);
}


