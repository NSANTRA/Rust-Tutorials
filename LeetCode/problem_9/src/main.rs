fn is_palindrome(x: i32) -> bool {
    let mut rev: i32 = 0;
    let mut temp: i32 = x;

    if x < 0 {
        return false;
    }
    
    while temp > 0 {
        rev = (rev * 10) + temp % 10;
        temp = temp / 10;
    }

    rev == x
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(true, is_palindrome(121));
    }

    #[test]
    fn case_2() {
        assert_eq!(false, is_palindrome(-121));
    }

    #[test]
    fn case_3() {
        assert_eq!(false, is_palindrome(10));
    }
}