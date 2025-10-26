// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        let p3 = power_of_2(3);
        let p5 = power_of_2(5);
        let p10 = power_of_2(10);
        let p0 = power_of_2(0);
        assert_eq!(p3, 8);
        assert_eq!(p5, 32);
        assert_eq!(p10, 1024);
        assert_eq!(p0, 1);
    }
}
