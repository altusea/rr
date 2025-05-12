use num_traits::Num;

pub fn gcd<T: Num + Clone>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b.clone(), (a % b).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12i32, 15i32), 3i32);
        assert_eq!(gcd(3i64, 7i64), 1i64);
    }
}
