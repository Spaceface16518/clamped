pub trait Clamp<T> {
    fn clamped(self, min: T, max: T) -> T;
}

impl<T> Clamp<T> for T where T: PartialOrd {
    #[cfg_attr(feature = "inline_always", inline(always))]
    fn clamped(self, min: T, max: T) -> T {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

#[cfg(test)]
#[cfg(feature = "clamp_trait")]
mod tests {
    use super::Clamp;

    #[test]
    fn test_lower_bound() {
        let value = 6.0;
        let min = 8.0;
        let max = 10.0;

        assert_eq!(value.clamped(min, max), min);
    }

    #[test]
    fn test_higher_bound() {
        let value = 30;
        let min = 0;
        let max = 10;

        assert_eq!(value.clamped(min, max), max);
    }

    #[test]
    fn test_inbounds() {
        let value = 15;
        let min = 3;
        let max = 20;

        assert_eq!(value.clamped(min, max), value);
    }

    macro_rules! test_type_integer {
        ($fn_name:ident, $t:ty, $low:expr, $high:expr, $low_test:expr, $high_test:expr) => {
            #[test]
            fn $fn_name() {
                let ltest: $t = $low_test;
                let htest: $t = $high_test;
                for value in ltest..htest {
                    let c: $t = value.clamped($low, $high);
                    assert!(c >= $low && c <= $high);
                    assert!(c == value || c == $low || c == $high)
                }
            }
        };
    }

    test_type_integer!(test_type_i8, i8, 20, 30, 0, 40);
    test_type_integer!(test_type_i16, i16, 20, 30, 0, 40);
    test_type_integer!(test_type_i32, i32, 20, 30, 0, 40);
    test_type_integer!(test_type_i64, i64, 20, 30, 0, 40);
    test_type_integer!(test_type_isize, isize, 20, 30, 0, 40);
    test_type_integer!(test_type_i128, i128, 20, 30, 0, 40);

    test_type_integer!(test_type_u8, u8, 20, 30, 0, 40);
    test_type_integer!(test_type_u16, u16, 20, 30, 0, 40);
    test_type_integer!(test_type_u32, u32, 20, 30, 0, 40);
    test_type_integer!(test_type_u64, u64, 20, 30, 0, 40);
    test_type_integer!(test_type_usize, usize, 20, 30, 0, 40);
    test_type_integer!(test_type_u128, u128, 20, 30, 0, 40);

    macro_rules! test_type_float {
        ($fn_name:ident, $t:ty, $low:expr, $high:expr, $low_test:expr, $high_test:expr) => {
        #[test]
            fn $fn_name() {
                let ltest = $low_test;
                let htest = $high_test;
                for v in ltest..htest {
                    let value = v as $t;
                    let c: $t = (value as $t).clamped($low, $high);
                    assert!(c >= $low && c <= $high);
                    assert!(c == value || c == $low || c == $high)
                }
            }
        };
    }

    test_type_float!(test_type_f32, f32, 20.0, 30.0, 0, 40);
    test_type_float!(test_type_f64, f64, 20.0, 30.0, 0, 40);
}
