use std::env;

fn main() {
    let mut s = String::new();
    for arg in env::args().skip(1) {
        s += &(arg + " ");
    }
    println!("{}", lagrange_string_interp(&s));
}

fn lagrange_string_interp(s: &str) -> String {
    let xs: Vec<f64> = (0..s.len()).map(|i| i as f64).collect();
    let ys = s.chars().map(|c| c as u8 as f64).collect::<Vec<f64>>();

    let mut result = String::from("(");

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
                numerator += &format!("(x-{:.}.0)*", xj);
            }
            denominator *= x - xj;
        }
        y /= denominator;

        numerator.pop();

        if y == 1.0 {
            result += &numerator;
        } else {
            if y.fract() == 0.0 {
                result += &format!("{:.}.0*{:.}", y, numerator);
            } else {
                result += &format!("{:.}*{:.}", y, numerator);
            }
        }
    }

    result.push_str(").round() as u8 as char");
    result.replace("+-", "-")
}
