use std::cmp;

struct PrimeIndex {
    step: u32,
    index: u16,
}

const L1D_CACHE_SIZE: usize = 32768;

pub fn segmented_sieve(limit: usize) -> usize {
    let sqrt = (limit as f64).sqrt() as usize;
    let sqrt_sqrt = (sqrt as f64).sqrt() as usize;
    let mut is_prime: Vec<bool> = vec![true; sqrt + 1];
    for i in (3..=sqrt_sqrt).step_by(2) {
		if is_prime[i] {
            for j in (i*i..=sqrt).step_by(i) {
                is_prime[j] = false;
            }
		}
    }

    let segment_size = cmp::max(sqrt, L1D_CACHE_SIZE);
    let mut count = if limit < 2 { 0 } else { 1 };
    let mut n = 3;
    let mut s = 3;
    let mut sieve: Vec<bool> = vec![true; segment_size];
    let mut prime_indexes: Vec<PrimeIndex> = Vec::new();

    for low in (0..=limit).step_by(segment_size) {
        sieve.fill(true);

        let high = cmp::min(low + segment_size - 1, limit);

        while s * s <= high {
            if is_prime[s] {
                prime_indexes.push(PrimeIndex {
                    step: (s * 2) as u32,
                    index: (s * s - low) as u16,
                });
            }
            s += 2;
        }

        for prime_index in prime_indexes.iter_mut() {
            let mut j = prime_index.index as usize;
            let step = prime_index.step as usize; // putting this struct field in a local variable improves program speed with 10%
            while j < segment_size {
                sieve[j] = false;
                j += step;
            }
            prime_index.index = (j - segment_size) as u16;
        }

        while n <= high {
            if sieve[n - low] {
                count += 1;
            }
            n += 2;
        }
    }

    count
}

#[test]
fn test_segmented_sieve() {
    assert_eq!(segmented_sieve(100), 25, "Sieve over a single segment");
    assert_eq!(segmented_sieve(1_000_000), 78498, "Sieve over multiple segments");
}
