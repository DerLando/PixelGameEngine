

/// Adds lhs and rhs, returns at maximum, the value specified
pub fn clamped_add(lhs: u32, rhs: u32, max: u32) -> u32 {
    let result = lhs + rhs;
    if result > max {max}
    else {result}
}

/// Subtracts rhs from lhs, returns at minimum, the value specified
pub fn clamped_sub(lhs: u32, rhs: u32, min: u32) {

}