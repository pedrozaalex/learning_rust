use std::collections::VecDeque;

fn main() {
    println!("{}", fib(1));
}

fn fib(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    let mut buf = VecDeque::from([0, 1]);

    for _ in 0..n - 2 {
        let sum = buf.iter().sum();
        buf.push_back(sum);
        buf.pop_front();
    }

    return buf.iter().sum();
}
