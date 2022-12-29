func felt_vs_integer(a: u128) -> (felt,u128) {
    let b = u128_to_felt(a);
    return (b,a);
}
