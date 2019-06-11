#![no_std]

#[cfg(feature = "clamp_fn")]
pub use clamp_fn::clamp;
#[cfg(feature = "clamp_trait")]
pub use clamp_trait::Clamp;

mod clamp_trait;
mod clamp_fn;

#[cfg_attr(feature = "clamp_macro", macro_export)]
macro_rules! clamp {
    ($val:expr, $min:expr, $max:expr) => {
        if $val < $min {
            $min
        } else if $val > $max {
            $max
        } else {
            $val
        }
    };
}

#[cfg(test)]
#[cfg(feature = "clamp_macro")]
mod macro_tests {
    #[test]
    fn test_lower_bound() {
        let value = 6;
        let min = 8;
        let max = 10;

        assert_eq!(clamp!(value, min, max), min);
    }

    #[test]
    fn test_higher_bound() {
        let value = 30;
        let min = 0;
        let max = 10;

        assert_eq!(clamp!(value, min, max), max);
    }

    #[test]
    fn test_inbounds() {
        let value = 15;
        let min = 3;
        let max = 20;

        assert_eq!(clamp!(value, min, max), value);
    }
}

