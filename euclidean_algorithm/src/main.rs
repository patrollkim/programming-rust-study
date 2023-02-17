use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER...");
        std::process::exit(1);
    } else {
        println!("numbers length : {}", numbers.len());
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);

}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }

    n
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(15, 10), 5);
    assert_eq!(gcd(12 * 3 * 4, 12 * 2 * 3), 12 * 3 * 2);
}