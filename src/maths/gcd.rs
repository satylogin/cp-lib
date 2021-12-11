/// Calculates gcd for same types T that implements required traits.
/// The current implementation is tested on i32, u32, i64, u64, usize, isize
/// For any other type T, the behaviour is undefined
pub fn gcd<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + Default + Copy,
{
    // for our required types, default evalutaes to 0 at compile time.
    // and thus have 0 cost abstraction.
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for &(x, y, g) in &[
            (1, 2, 1),
            (0, 1, 1),
            (0, 9, 9),
            (1, 1, 1),
            (1, 9, 1),
            (2, 8, 2),
            (6, 9, 3),
        ] {
            assert_eq!(g as i32, gcd(x as i32, y as i32));
            assert_eq!(g as u32, gcd(x as u32, y as u32));
            assert_eq!(g as i64, gcd(x as i64, y as i64));
            assert_eq!(g as u64, gcd(x as u64, y as u64));
            assert_eq!(g as usize, gcd(x as usize, y as usize));
            assert_eq!(g as isize, gcd(x as isize, y as isize));
        }
    }
}
