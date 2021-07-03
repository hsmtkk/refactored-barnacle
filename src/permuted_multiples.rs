fn contain_same_digit(x:i64, y:i64) -> bool {
    let s = x.to_string();
    let t = y.to_string();
    let mut ss: Vec<char> = s.chars().collect();
    let mut ts: Vec<char> = t.chars().collect();
    ss.sort_unstable();
    ts.sort_unstable();
    ss == ts
}

fn find_solution() -> i64 {
    let mut i = 1;
    loop {
        let mut ok = true;
        for j in 1..7 {
            let k = i * j;
            if ! contain_same_digit(i, k) {
                ok = false;
                break;
            }
        }
        if ok {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_contain_same_digit(){
        assert_eq!(true, super::contain_same_digit(125874, 251748));
        assert_eq!(false, super::contain_same_digit(123456, 234567));
    }

    #[test]
    fn test_find_solution(){
        assert_eq!(142857, super::find_solution());
    }
}
