// simple recursive fibonacci function
pub fn one(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return one(n - 1) + one(n - 2);
}

// simple iterative fibonacci function
pub fn two(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    return c;
}

// recursive fibonacci function with memoization
// requires o(n) input space
pub fn three(n: i32, memo: &mut Vec<i32>) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if memo[n as usize] != 0 {
        return memo[n as usize];
    }
    memo[n as usize] = three(n - 1, memo) + three(n - 2, memo);
    return memo[n as usize];
}

// fibonacci matrix exponentiation
pub fn four(n: i32) -> i32 {
    // unfortunately requires external crates to be efficient
    // but essentially its just a = [[1, 1], [1, 0]] ^ n
    // then return a[0][0]
    0
}

// generalization of fibonacci
// https://en.wikipedia.org/wiki/Generalizations_of_Fibonacci_numbers#Higher_orders
// THIS IS NOT BINET'S FORMULA (though similar)
pub fn five(n: i32) -> i32 {
    let r = (1.0 + 5.0f64.sqrt()) / 2.0; // golden ratio
    let numerator = (r.powi(n - 1)) * (r - 1.0); // possible not o(1), powi is tricky
    let denominator = 3.0 * r - 4.0; // fibo = 2, (f + 1)r - 2f
    (numerator / denominator).round() as i32
}


impl Solution {
    pub fn fib(n: i32) -> i32 {
        five(n)
    }
}