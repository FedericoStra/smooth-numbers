/*! Algorithms to generate [smooth numbers](https://mathworld.wolfram.com/SmoothNumber.html).

# Examples

Generate the first `10` numbers of the form `2^j * 3^j`:
```
use smooth_numbers::pratt;
assert_eq!(pratt(10), [1, 2, 3, 4, 6, 8, 9, 12, 16, 18]);
```

Generate the first `10` numbers of the form `2^j * 3^j * 5^k`:
```
use smooth_numbers::smooth;
assert_eq!(smooth(5, 10), [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]);
```
*/

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/** Generates the first `n` numbers in the Pratt's sequence.

Pratt's sequence is the sequence of numbers of the form `2^i * 3^j`,
with `i` and `j` natural numbers. These are also called [3-smooth numbers] and
they are indexed in OEIS as [A003586](https://oeis.org/A003586).

They provide the best-known sequence of gaps for [Shellsort] (best worst-case time complexity).

[`pratt`]`(n)` is equivalent to [`smooth`]`(3, n)`, but faster.

[3-smooth numbers]: https://mathworld.wolfram.com/SmoothNumber.html
[Shellsort]: https://en.wikipedia.org/wiki/Shellsort#Gap_sequences
*/
pub fn pratt(n: usize) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }

    let mut v = Vec::with_capacity(n);
    v.push(1);

    let mut two = 0;
    let mut three = 0;

    for _ in 1..n {
        let times_two = 2 * v[two];
        let times_three = 3 * v[three];
        match (times_two).cmp(&times_three) {
            std::cmp::Ordering::Less => {
                v.push(times_two);
                two += 1;
            }
            std::cmp::Ordering::Equal => {
                v.push(times_two);
                two += 1;
                three += 1;
            }
            std::cmp::Ordering::Greater => {
                v.push(times_three);
                three += 1;
            }
        }
    }
    v
}

/** Generates the first `n` `k`-smooth numbers, i.e. numbers whose prime factors
  are smaller than or equal to `k`.

See the definition of *smooth number* on
[Wikipedia](https://en.wikipedia.org/wiki/Smooth_number) and
[MathWorld](https://mathworld.wolfram.com/SmoothNumber.html).

# Examples

With `k < 2`, the numbers cannot have any prime factor,
hence the only smooth number in this case is `1`.

```
# use smooth_numbers::smooth;
assert_eq!(smooth(0, 0), []);
assert_eq!(smooth(0, 1), [1]);
assert_eq!(smooth(0, 10), [1]);
assert_eq!(smooth(1, 10), [1]);
```

With `k == 2`, the numbers can only have the prime factor `2`,
hence we obtain the sequence of the powers of `2`.

```
# use smooth_numbers::smooth;
assert_eq!(smooth(2, 10), [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]);
```

With `k == 3`, the numbers can only have the prime factors `2` and `3`,
hence we obtain the sequence of the numbers of the form `2^i * 3^j`,
also known as Pratt's sequence.
See the [`pratt`] function for a specialized algorithm to generate this sequence.

```
# use smooth_numbers::smooth;
assert_eq!(smooth(3, 10), [1, 2, 3, 4, 6, 8, 9, 12, 16, 18]);
```
*/
pub fn smooth(k: usize, n: usize) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }

    if k < 2 {
        return vec![1];
    }

    if k == 2 {
        let mut v = Vec::with_capacity(n);
        v.push(1);
        let mut x = 1;
        for _ in 1..n {
            x *= 2;
            v.push(x);
        }
        return v;
    }

    let sieve = primal::Sieve::new(k);
    let primes: Vec<u64> = sieve
        .primes_from(2)
        .take_while(|&p| p <= k)
        .map(|p| p as u64)
        .collect();
    let num_primes = primes.len();
    let mut indices: Vec<usize> = vec![0; num_primes];
    let mut v = Vec::with_capacity(n);
    v.push(1);

    for _ in 1..n {
        let i = (0..num_primes)
            .min_by_key(|&j| primes[j] * v[indices[j]])
            .expect("cannot find next index");
        let new = primes[i] * v[indices[i]];
        v.push(new);
        for j in 0..num_primes {
            if primes[j] * v[indices[j]] == new {
                indices[j] += 1;
            }
        }
    }
    v
}

/** Generates the first `n` smooth numbers whose prime factors are among `primes`.

# Examples

If `primes` is empty, the numbers cannot have any prime factor,
hence the only smooth number in this case is `1`.

```
# use smooth_numbers::with_primes;
assert_eq!(with_primes(&[], 0), []);
assert_eq!(with_primes(&[], 1), [1]);
assert_eq!(with_primes(&[], 10), [1]);
```

If `primes` contains a single element, the numbers can only have the prime factor `primes[0]`,
hence we obtain the sequence of the powers of `primes[0]`.

```
# use smooth_numbers::with_primes;
assert_eq!(with_primes(&[2], 10), [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]);
```

If `primes == &[2, 3]`, the numbers can only have the prime factors `2` and `3`,
hence we obtain the sequence of the numbers of the form `2^i * 3^j`,
also known as Pratt's sequence.
See the [`pratt`] function for a specialized algorithm to generate this sequence.

```
# use smooth_numbers::with_primes;
assert_eq!(with_primes(&[2, 3], 10), [1, 2, 3, 4, 6, 8, 9, 12, 16, 18]);
```
*/
pub fn with_primes(primes: &[u64], n: usize) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }

    let num_primes = primes.len();

    if num_primes == 0 {
        return vec![1];
    }

    if num_primes == 1 {
        let mut v = Vec::with_capacity(n);
        v.push(1);
        let mut x = 1;
        for _ in 1..n {
            x *= primes[0];
            v.push(x);
        }
        return v;
    }

    let mut indices: Vec<usize> = vec![0; num_primes];
    let mut v = Vec::with_capacity(n);
    v.push(1);

    for _ in 1..n {
        let i = (0..num_primes)
            .min_by_key(|&j| primes[j] * v[indices[j]])
            .expect("cannot find next index");
        let new = primes[i] * v[indices[i]];
        v.push(new);
        for j in 0..num_primes {
            if primes[j] * v[indices[j]] == new {
                indices[j] += 1;
            }
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pratt_has_correct_length() {
        for n in [0, 1, 10, 100] {
            assert_eq!(pratt(n).len(), n);
        }
    }

    #[test]
    fn pratt_has_correct_values() {
        assert_eq!(pratt(10), [1, 2, 3, 4, 6, 8, 9, 12, 16, 18]);
        assert_eq!(pratt(100).last(), Some(&93312));
        // this is the largest possible
        assert_eq!(pratt(1343).last(), Some(&17748888853923495936));
        assert_eq!(pratt(1344).last(), Some(&17991041643939889152));
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    // #[should_panic(expected = "index out of bounds")] // with `overflow-checks = false`
    fn pratt_first_to_overflow() {
        let _ = pratt(1345).last();
    }

    #[test]
    fn smooth_has_correct_length() {
        for n in [0, 1, 10, 64] {
            assert_eq!(smooth(2, n).len(), n);
        }
        for n in [0, 1, 10, 100] {
            assert_eq!(smooth(3, n).len(), n);
            assert_eq!(smooth(5, n).len(), n);
        }
    }

    #[test]
    fn smooth_has_correct_values() {
        assert_eq!(smooth(2, 10), [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]);
        assert_eq!(smooth(3, 10), [1, 2, 3, 4, 6, 8, 9, 12, 16, 18]);
        assert_eq!(smooth(3, 100).last(), Some(&93312));
        // this is the largest possible
        assert_eq!(smooth(3, 1343).last(), Some(&17748888853923495936));
        assert_eq!(smooth(3, 1344).last(), Some(&17991041643939889152));
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn smooth_1_first_to_overflow() {
        let _ = smooth(2, 65).last();
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn smooth_2_first_to_overflow() {
        let _ = smooth(3, 1345).last();
    }

    #[test]
    fn with_primes_has_correct_length() {
        for n in [0, 1, 10, 64] {
            assert_eq!(with_primes(&[2], n).len(), n);
        }
        for n in [0, 1, 10, 100] {
            assert_eq!(with_primes(&[2, 3], n).len(), n);
            assert_eq!(with_primes(&[2, 3, 5], n).len(), n);
        }
    }

    #[test]
    fn with_primes_has_correct_values() {
        assert_eq!(
            with_primes(&[2], 10),
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]
        );
        assert_eq!(with_primes(&[2, 3], 10), [1, 2, 3, 4, 6, 8, 9, 12, 16, 18]);
        assert_eq!(with_primes(&[2, 3], 100).last(), Some(&93312));
        // this is the largest possible
        assert_eq!(
            with_primes(&[2, 3], 1343).last(),
            Some(&17748888853923495936)
        );
        assert_eq!(
            with_primes(&[2, 3], 1344).last(),
            Some(&17991041643939889152)
        );
        assert_eq!(
            with_primes(&[2, 5], 10),
            [1, 2, 4, 5, 8, 10, 16, 20, 25, 32]
        );
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn with_primes_1_first_to_overflow() {
        let _ = with_primes(&[2], 65).last();
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn with_primes_2_first_to_overflow() {
        let _ = with_primes(&[2, 3], 1345).last();
    }
}
