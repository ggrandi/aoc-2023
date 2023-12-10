pub fn char_to_usize(c: char) -> usize {
    ((c as u8) - b'0') as usize
}

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
    #[cfg(debug_assertions)]
    { println!($($arg)*);}};
}

pub trait GreatestCommonDenominator {
    fn gcd(&self, other: &Self) -> Self;
}

pub trait LeastCommonMultiple {
    fn lcm(&self, other: &Self) -> Self;
}

macro_rules! GCD_LCM_Impl {
    ($($t:ty $(,)?)*) => {
        $(
impl GreatestCommonDenominator for $t {
    fn gcd(&self, other: &Self) -> Self {
        if *other == 0 {
            return *self;
        }

        other.gcd(&(self % other))
    }
}

impl LeastCommonMultiple for $t {
    fn lcm(&self, other: &Self) -> Self {
        (*self * *other) / self.gcd(other)
    }
})*
    };
}

GCD_LCM_Impl!(usize, u64, u32, u16, u8);

#[cfg(test)]
mod tests {
    use super::GreatestCommonDenominator;

    #[test]
    fn gcd_test() {
        assert_eq!(8u16.gcd(&4), 4);
        assert_eq!(8u16.gcd(&0), 8);
        assert_eq!(0u16.gcd(&8), 8);
        assert_eq!(54u8.gcd(&24), 6);
    }
}
