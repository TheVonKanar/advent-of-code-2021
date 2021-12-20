pub fn abs_diff(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        rhs - lhs
    } else {
        lhs - rhs
    }
}
