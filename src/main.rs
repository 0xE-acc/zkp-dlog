use num_bigint::{BigUint};
use rand::Rng;
use primal::is_prime;

fn generate_large_prime(bits: usize) -> BigUint {
    // generate random number with desired number of bits
    let mut rng = rand::thread_rng();
    let candidate = rng.gen_biguint(bits as u64);

    // use primality test to find next prime >= candidate
    let prime = loop {
        if is_prime(&candidate.to_u64().unwrap()) {
            break candidate.clone();
        }
        candidate.clone_inc();
    };
    prime
}

fn dlogProof1(x: i32, g: i32, p: i32) -> [i32] {
    // step one of proof
    let mut y = x.pow(g as u32);
    y = y % p;

    y
}

fn main() {
    let rand_generator = rand::thread_rng();
    println!("Generator: {}", rand_generator);
    let secret_id = rand::thread_rng();
    println!("Secret ID: {}", secret_id);
    let large_prime = generate_large_prime(4);
    println!("Large Prime: {}", large_prime);

    // prover sends y = x^g(mod p)
    println!("Prover calculating y = rand_generator ^ secret_id");
    let h = dlogProof1(rand_generator, secret_id, large_prime);
    
    // verifier sends back random bit 
    println!("Verifier sending random bit...");
    let mut verifier_random_bit = rand::thread_rng();
    verifier_random_bit.gen_range(0..=1);

    // prover sends s = (rand_generator + verifier_random_bit * secret_id)(mod (large_prime - 1))
    println!("Prover sending s = (rand_generator + verifier_random_bit * secret_id)(mod (large_prime - 1))");
    let mut s = rand_generator + verifier_random_bit * secret_id;
    s = s % (large_prime - 1);

    println!("Verifier computing rand_generator ^ s(mod large_prime)");
    let mut verifier_solution = rand_generator.pow(s);
    verifier_solution = verifier_solution % large_prime;
    
    println!("verifier_solution should equal h^verifer_random_bit(mod large_prime)");
    let compared_solution = h.pow(verifier_random_bit) % large_prime;
    println!("verifier_solution: {}, compared_solution: {}", verifier_solution, compared_solution);
}
