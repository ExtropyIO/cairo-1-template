func power(base: felt, exp: felt) -> felt {
    if (exp == 0) {
        return 1;
    }

    let res = power(base, exp - 1);
    return res * base;
}
