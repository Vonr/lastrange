use std::env;

fn main() {
    let mut s = String::new();
    for arg in env::args().skip(1) {
        s += &(arg + " ");
    }
    println!("{}", lagrange_string_interp(&s));
    println!();
}

fn lagrange_string_interp(s: &str) -> String {
    let xs: Vec<f64> = (0..s.len()).map(|i| i as f64).collect();
    let ys = s.chars().map(|c| c as u8 as f64).collect::<Vec<f64>>();

    let mut result = String::new();

    for (i, x) in xs.iter().enumerate() {
        let mut y = ys[i];
        let mut numerator = String::new();
        let mut denominator = 1f64;

        if i > 0 {
            result.push_str("+");
        }

        for (j, xj) in xs.iter().enumerate() {
            if i == j {
                continue;
            }

            if xj == &0.0 {
                numerator += &format!("x*");
            } else {
                numerator += &format!("(x-{}.0)*", xj);
            }
            denominator *= x - xj;

            let gcd = gcd(denominator, y);
            y /= gcd;
            denominator /= gcd;
        }

        numerator.pop();

        if denominator == 1.0 {
            if y == 1.0 {
                result += &numerator;
            } else {
                result += &format!("{}.0*{}", y, numerator);
            }
        } else {
            if y == 1.0 {
                result += &format!("{}/{}.0", numerator, denominator);
            } else {
                result += &format!("{}.0*{}/{}.0", y, numerator, denominator);
            }
        }
    }

    result.replace("+-", "-")
}

fn gcd(mut a: f64, mut b: f64) -> f64 {
    while b != 0.0 {
        (a, b) = (b, a % b);
    }
    a
}
