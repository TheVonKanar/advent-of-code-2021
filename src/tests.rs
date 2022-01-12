#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use crate::processors::*;

    fn test<R1, R2>(processor: fn() -> (R1, R2), result: (R1, R2)) -> bool
    where
        R1: Display + PartialEq,
        R2: Display + PartialEq,
    {
        processor() == result
    }

    #[test]
    fn day01() {
        test(day01::processor, (1559, 1600));
    }

    #[test]
    fn day02() {
        test(day02::processor, (1427868, 1568138742));
    }

    #[test]
    fn day03() {
        test(day03::processor, (3901196, 4412188));
    }

    #[test]
    fn day04() {
        test(day04::processor, (8136, 12738));
    }

    #[test]
    fn day05() {
        test(day05::processor, (5632, 22213));
    }

    #[test]
    fn day06() {
        test(day06::processor, (372300, 1675781200288));
    }

    #[test]
    fn day07() {
        test(day07::processor, (336120, 96864235));
    }

    #[test]
    fn day08() {
        test(day08::processor, (493, 1010460));
    }

    #[test]
    fn day09() {
        test(day09::processor, (502, 1330560));
    }

    #[test]
    fn day10() {
        test(day10::processor, (344193, 3241238967));
    }

    #[test]
    fn day11() {
        test(day11::processor, (1688, 403));
    }

    #[test]
    fn day12() {
        test(day12::processor, (5252, 147784));
    }

    #[test]
    fn day13() {
        test(day13::processor, (695, ""));
    }

    #[test]
    fn day14() {
        test(day14::processor, (3831, 5725739914282));
    }

    #[test]
    fn day15() {
        test(day15::processor, (609, 2925));
    }

    #[test]
    fn day16() {
        test(day16::processor, (901, 110434737925));
    }

    #[test]
    fn day17() {
        test(day17::processor, (6903, 2351));
    }
}
