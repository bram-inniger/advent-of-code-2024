pub mod clique;
pub mod graph;
pub mod union_find;

#[allow(dead_code)]
pub const BASE_10: u32 = 10;

#[allow(dead_code)]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[allow(dead_code)]
pub fn gcd(a: u64, b: u64) -> u64 {
    let mut pair = (a, b);

    while pair.1 > 0 {
        pair = (pair.1, pair.0 % pair.1)
    }

    pair.0
}
