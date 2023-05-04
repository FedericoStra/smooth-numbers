//! Print all the 7-smooth numbers less than `2^64`.

use smooth_numbers::smooth;
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout().lock();
    for (i, n) in smooth(7, 85_348).iter().enumerate() {
        // ignore writing errors
        let _ = writeln!(stdout, "{:5}: {}", i + 1, n);
    }
}
