//! Print all the 5-smooth numbers less than `2^64`.

use smooth_numbers::smooth;
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout().lock();
    for (i, n) in smooth(5, 13_282).iter().enumerate() {
        // ignore writing errors
        let _ = writeln!(stdout, "{:5}: {}", i + 1, n);
    }
}
