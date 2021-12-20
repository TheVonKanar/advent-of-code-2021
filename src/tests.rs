#[cfg(test)]
mod tests {
    use crate::processors::*;

    #[test]
    fn day1() {
        assert_eq!(day1::process(), (1559, 1600));
    }

    #[test]
    fn day2() {
        assert_eq!(day2::process(), (1427868, 1568138742));
    }

    #[test]
    fn day3() {
        assert_eq!(day3::process(), (3901196, 4412188));
    }

    #[test]
    fn day4() {
        assert_eq!(day4::process(), (8136, 12738));
    }

    #[test]
    fn day5() {
        assert_eq!(day5::process(), (5632, 22213));
    }

    #[test]
    fn day6() {
        assert_eq!(day6::process(), (372300, 1675781200288));
    }

    #[test]
    fn day7() {
        assert_eq!(day7::process(), (336120, 96864235));
    }
}
