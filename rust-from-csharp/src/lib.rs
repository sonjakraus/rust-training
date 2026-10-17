#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn half_10_returns_5() {
        let result = half(10).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn half_0_returns_None() {
        let result = half(0);
        assert_eq!(result, None)
    }
}

/// Halfs a number
///
///
/// let result = rust_from_charp_lib::half(8).unwrap();
/// assert_eq!(result, 4);
/// 

pub fn half(num: i32) -> Option<i32> {
   if num == 0 {
    return None;
   } 
   Some (num/2)
}