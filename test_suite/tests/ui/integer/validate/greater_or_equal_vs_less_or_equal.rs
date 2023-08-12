use nutype::nutype;

#[nutype(validate(less_or_equal = 0, greater_or_equal = 20))]
pub struct Amount(i64);

fn main() {}
