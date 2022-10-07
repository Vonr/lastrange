use std::{cmp::Ordering, env, i128};

use rug::{Complete, Integer, Rational};

fn main() {
    let mut s = String::new();
    for arg in env::args().skip(1) {
        s += &(arg + " ");
    }
    println!("{}", lagrange_string_interp(&s));
}

fn lagrange_string_interp(s: &str) -> String {
    let one: Rational = Rational::from(1);
    let xs: Vec<i128> = (0..s.len()).map(|i| i as i128).collect();
    let ys: Vec<i128> = s.chars().map(|c| c as i128).collect();

    let mut result = String::with_capacity(256);

    let mut numerator = String::new();
    for ((i, x), y) in xs.iter().enumerate().zip(ys) {
        if i > 0 {
            result.push('+');
        }

        let mut denominator = Integer::from(1);
        xs.iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .for_each(|(_, xj)| {
                match *xj == 0 {
                    true => numerator += "x*",
                    false => numerator += &format!("(x-{:.}.0)*", xj),
                };
                denominator *= x - xj
            });
        let y = Rational::from(y) / denominator;

        numerator.pop();

        match y == one {
            true => result += &numerator,
            false if y.rem_trunc_ref().complete().cmp0() == Ordering::Equal => {
                result += &format!("{:.}.0*{:.}", y.to_f64(), numerator)
            }
            _ => result += &format!("{:.}*{:.}", y.to_f64(), numerator),
        }
        numerator.clear()
    }

    result.replace("+-", "-")
}
