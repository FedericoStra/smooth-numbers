//! Print all the 3-smooth numbers less than `2^64`.

use smooth_numbers::pratt;

fn main() {
    for n in pratt(1344) {
        println!("{n}");
    }
}
