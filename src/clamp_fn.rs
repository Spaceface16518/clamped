#[cfg_attr(feature = "inline_always", inline(always))]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
#[cfg(feature = "clamp_fn")]
mod tests {
    use crate::clamp;

    #[test]
    fn test_lower_bound() {
        let value = 6;
        let min = 8;
        let max = 10;

        assert_eq!(clamp(value, min, max), min);
    }

    #[test]
    fn test_higher_bound() {
        let value = 30;
        let min = 0;
        let max = 10;

        assert_eq!(clamp(value, min, max), max);
    }

    #[test]
    fn test_inbounds() {
        let value = 15;
        let min = 3;
        let max = 20;

        assert_eq!(clamp(value, min, max), value);
    }
}