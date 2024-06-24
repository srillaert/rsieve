mod segmented_sieve;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} STOP", args[0]);
        println!("Counts the primes inside [2, STOP] (< 2^30)");
        return;
    }
    let stop = args[1].parse().expect("STOP cannot be parsed");
    if stop >= (1 << 30) {
        panic!("STOP is too large");
    }
    let count = segmented_sieve::segmented_sieve(stop);
    println!("{}", count);
}