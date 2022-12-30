func felt_vs_integer(dividend: u128, divisor: u128, felt_dividend: felt, felt_divisor: NonZero::<felt>) -> (u128, felt) {
    let result = u128_div(dividend, divisor);
    let felt_result = felt_div(felt_dividend, felt_divisor);
    return (result, felt_result);
}
