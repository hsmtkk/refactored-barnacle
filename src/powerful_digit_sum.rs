use num_bigint::BigUint;

fn solve() -> u32{
    let mut max = 0;
    for a in 1..101 {
        for b in 1..101 {
            let big_a = BigUint::from(a as u32);
            let ans = big_a.pow(b);
            let s = sum_digit(ans);
            if s > max {
                max = s;
            }
        }
    }
    max
}

fn sum_digit(n:BigUint) -> u32 {
    let mut sum = 0;
    for d in n.to_str_radix(10).chars() {
        sum += d.to_digit(10).unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    #[test]
    fn test_sum_digit(){
        let n = BigUint::from(123 as u32);
        assert_eq!(6, super::sum_digit(n));
    }

    #[test]
    fn test_solution(){
        assert_eq!(972, super::solve());
    }
}