func felt_vs_integer(dividend: u128, divisor: u128) -> (felt,u128) {
    let felt_dividend = u128_to_felt(dividend);
    let felt_divisor: NonZero::<felt> = u128_checked_as_non_zero(u128_to_felt(divisor));
    let result = u128_div(dividend, divisor);
    let felt_result = felt_dividend / felt_divisor;
    return (felt_divisor,result);
}
