//! Print all the 3-smooth numbers less than `2^64`.

use smooth_numbers::smooth;
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout().lock();
    for (i, n) in smooth(3, 1_344).iter().enumerate() {
        // ignore writing errors
        let _ = writeln!(stdout, "{:4}: {}", i + 1, n);
    }
}
