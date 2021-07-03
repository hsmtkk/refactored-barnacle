use std::collections::HashMap;

trait Calculator {
    fn fact(&mut self, n: i32) -> i128;
    fn ncr(&mut self, n: i32, r: i32) -> i128;
}

fn new() -> impl Calculator {
    CalculatorImpl::new()
}

struct CalculatorImpl {
    memo: HashMap<i32, i128>,
}

impl CalculatorImpl {
    fn new() -> CalculatorImpl {
        CalculatorImpl {
            memo: HashMap::new(),
        }
    }
}

impl Calculator for CalculatorImpl {
    fn fact(&mut self, n: i32) -> i128 {
        if let Some(m) = self.memo.get(&n) {
            return *m;
        }
        match n {
            0 => 1,
            _ => {
                let m = n as i128 * self.fact(n - 1);
                self.memo.insert(n, m);
                m
            }
        }
    }

    fn ncr(&mut self, n: i32, r: i32) -> i128 {
        self.fact(n) / self.fact(r) / self.fact(n - r)
    }
}

fn find_solution() -> i32 {
    let mut count = 0;
    let mut calc = new();
    for n in 1..101 {
        for r in 1..n + 1 {
            let m = calc.ncr(n, r);
            if m > 1000000 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::combinatoric_selections::Calculator;

    #[test]
    fn test_fact() {
        let mut calc = super::new();
        assert_eq!(120, calc.fact(5));
    }

    #[test]
    fn test_ncr() {
        let mut calc = super::new();
        assert_eq!(10, calc.ncr(5, 3));
    }

    #[test]
    fn test_find_solution() {
        assert_eq!(4075, super::find_solution());
    }
}
