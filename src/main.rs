mod segmented_sieve;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let limit: usize = if args.len() >= 2 {
        args[1].parse().expect("Invalid number")
    } else {
        1_000_000
    };

    let count = segmented_sieve::segmented_sieve(limit);
    println!("{} primes found.", count);
}
