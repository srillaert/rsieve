use std::cmp;

struct PrimeIndex {
    prime: u32,
    segment_index: u32,
}

const L1D_CACHE_SIZE: u32 = 32768;

fn get_is_prime(inclusive_till: u32) -> Vec<bool> {
    let sqrt = (inclusive_till as f64).sqrt() as usize;
    let mut is_prime: Vec<bool> = vec![true; (inclusive_till as usize) + 1];
    for i in (3..=sqrt).step_by(2) {
        if is_prime[i] {
            for j in (i*i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

fn integer_sqrt(n: usize) -> u32 {
    (n as f64).sqrt() as u32
}

#[test]
fn test_integer_sqrt() {
    assert_eq!(integer_sqrt(0), 0);
    assert_eq!(integer_sqrt(4), 2);
    assert_eq!(integer_sqrt(usize::MAX), u32::MAX, "sqrt of usize::MAX is u32::MAX");
    assert!((u32::MAX as usize) * (u32::MAX as usize) < usize::MAX, "but the square of u32::MAX is less than usize::MAX");
}

pub fn segmented_sieve(limit: usize) -> usize {
    let sqrt = integer_sqrt(limit);
    let is_prime = get_is_prime(sqrt);

    let segment_size = L1D_CACHE_SIZE as usize;
    let segment_count = (limit / segment_size) + 1;
    let mut count = 0;
    let mut s = 3;
    let mut sieve: Vec<bool> = vec![true; segment_size];
    let mut prime_indexes: Vec<PrimeIndex> = Vec::new();

    let mut segment_index = 0usize;
    loop {
        let low = segment_index * segment_size;
        let high = cmp::min(low + segment_size - 1, limit);

        while s * s <= high {
            if is_prime[s] {
                prime_indexes.push(PrimeIndex {
                    prime: s as u32,
                    segment_index: (s * s - low) as u32,
                });
            }
            s += 2;
        }

        for prime_index in prime_indexes.iter_mut() {
            let mut j = prime_index.segment_index as usize;
            let step = (prime_index.prime as usize) * 2;
            while j < segment_size {
                sieve[j] = false;
                j += step;
            }
            prime_index.segment_index = (j - segment_size) as u32;
        }

        for n in ((low+1)..=high).step_by(2) {
            if sieve[n - low] {
                count += 1;
            }
        }

        segment_index += 1;
        if segment_index > segment_count {
            break count;
        }
        sieve.fill(true);
    }
}

#[test]
fn test_segmented_sieve() {
    assert_eq!(segmented_sieve(100), 25, "Sieve over a single segment");
    assert_eq!(segmented_sieve(1_000_000), 78498, "Sieve over multiple segments");
}
