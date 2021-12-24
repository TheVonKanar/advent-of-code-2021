#[cfg(test)]
mod tests {
    use crate::processors::*;

    #[test]
    fn day01() {
        assert_eq!(day01::process(), (1559, 1600));
    }

    #[test]
    fn day02() {
        assert_eq!(day02::process(), (1427868, 1568138742));
    }

    #[test]
    fn day03() {
        assert_eq!(day03::process(), (3901196, 4412188));
    }

    #[test]
    fn day04() {
        assert_eq!(day04::process(), (8136, 12738));
    }

    #[test]
    fn day05() {
        assert_eq!(day05::process(), (5632, 22213));
    }

    #[test]
    fn day06() {
        assert_eq!(day06::process(), (372300, 1675781200288));
    }

    #[test]
    fn day07() {
        assert_eq!(day07::process(), (336120, 96864235));
    }

    #[test]
    fn day08() {
        assert_eq!(day08::process(), (493, 1010460));
    }

    #[test]
    fn day09() {
        assert_eq!(day09::process(), (502, 1330560));
    }

    #[test]
    fn day10() {
        assert_eq!(day10::process(), (344193, 3241238967));
    }
}
