#![feature(test)]

extern crate test;

fn main() {
    print!("{}", mul0(15, 14));
}

fn mul_acc4(mut a: i32, mut b: i32, mut r: i32) -> i32 {
    loop {
        if odd(a) {
            r += b;

            if a == 1 {
                return r;
            }
        }

        a = half(a);
        b = b + b;
    }
}

fn mul_acc3(mut a: i32, mut b: i32, mut r: i32) -> i32 {
    if odd(a) {
        if a == 1 {
            return a;
        }

        r += b;
    }

    a = half(a);
    b = b + b;

    mul_acc1(a, b, r)
}

fn mul_acc2(a: i32, b: i32, mut r: i32) -> i32 {
    if odd(a) {
        if a == 1 {
            return a;
        }

        r += b;
    }

    mul_acc1(half(a), b + b, r)
}

fn mul_acc1(a: i32, b: i32, r: i32) -> i32 {
    if a == 1 {
        return b + r;
    }

    let acc = if odd(a) { r + b } else { r };

    mul_acc1(half(a), b + b, acc)
}

fn mul_acc0(a: i32, b: i32, r: i32) -> i32 {
    if a == 1 {
        return b + r;
    }

    if odd(a) {
        return mul_acc0(half(a), b + b, r + b);
    }

    mul_acc0(half(a), b + b, r)
}

fn mul0(a: i32, b: i32) -> i32 {
    if a == 1 {
        b
    } else {
        mul0(a - 1, b) + b
    }
}

fn mul1(a: i32, b: i32) -> i32 {
    if a == 1 {
        return b;
    }

    let result = mul1(half(a), b + b);

    if odd(a) {
        result + b
    } else {
        result
    }
}

fn mul2(a: i32, b: i32) -> i32 {
    if a == 1 {
        b
    } else {
        mul_acc4(a - 1, b, b)
    }
}

fn mul3(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    while !odd(a) {
        b = b + b;
        a = half(a);
    }

    if a == 1 {
        return b;
    }

    mul_acc4(a - 1, b, b)
}

fn mul4(mut a: i32, mut b: i32) -> i32 {
    while !odd(a) {
        b = b + b;
        a = half(a);
    }

    if a == 1 {
        return b;
    }

    mul_acc4(half(a - 1), b + b, b)
}

fn half(a: i32) -> i32 {
    a >> 1
}

fn odd(a: i32) -> bool {
    a & 0x1 == 1
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_mul0() {
        assert_eq!(42, mul0(2, 21));
        assert_eq!(42, mul0(21, 2));
    }

    #[test]
    fn test_mul1() {
        assert_eq!(42, mul1(2, 21));
        assert_eq!(42, mul1(21, 2));
    }

    #[test]
    fn test_mul_acc0() {
        assert_eq!(42, mul_acc0(2, 21, 0));
        assert_eq!(42, mul_acc0(21, 2, 0));
    }

    #[test]
    fn test_mul_acc1() {
        assert_eq!(42, mul_acc1(2, 21, 0));
        assert_eq!(42, mul_acc1(21, 2, 0));
    }

    #[test]
    fn test_mul_acc2() {
        assert_eq!(42, mul_acc2(2, 21, 0));
        assert_eq!(42, mul_acc2(21, 2, 0));
    }

    #[test]
    fn test_mul_acc3() {
        assert_eq!(42, mul_acc3(2, 21, 0));
        assert_eq!(42, mul_acc3(21, 2, 0));
    }
    #[test]
    fn test_mul_acc4() {
        assert_eq!(42, mul_acc4(2, 21, 0));
        assert_eq!(42, mul_acc4(21, 2, 0));
        assert_eq!(56, mul_acc4(7, 8, 0));
        assert_eq!(56, mul_acc4(8, 7, 0));
    }

    #[test]
    fn test_mul2() {
        assert_eq!(42, mul2(2, 21));
        assert_eq!(42, mul2(21, 2));
    }

    #[test]
    fn test_mul3() {
        assert_eq!(42, mul3(2, 21));
        assert_eq!(42, mul3(21, 2));
    }

    #[test]
    fn test_mul4() {
        assert_eq!(42, mul4(2, 21));
        assert_eq!(42, mul4(21, 2));
    }
    #[bench]
    fn bench_mul0(b: &mut Bencher) {
        b.iter(|| {
            let arg = 42000;
            let mut acc = 0;

            for i in 1..100000 {
                acc += black_box(mul0(arg, i));
            }

            return acc;
        });
    }

    #[bench]
    fn bench_mul1(b: &mut Bencher) {
        b.iter(|| {
            let arg = 42000;
            let mut acc = 0;

            for i in 1..100000 {
                acc += mul1(arg, i);
            }

            return acc;
        });
    }

    #[bench]
    fn bench_mul2(b: &mut Bencher) {
        b.iter(|| {
            let arg = 42000;
            let mut acc = 0;
            for i in 1..100000 {
                acc += mul2(arg, i);
            }

            return acc;
        });
    }

    #[bench]
    fn bench_mul3(b: &mut Bencher) {
        b.iter(|| {
            let arg = 42000;
            let mut acc = 0;
            for i in 1..100000 {
                acc += mul3(arg, i);
            }

            return acc;
        });
    }

    #[bench]
    fn bench_mul4(b: &mut Bencher) {
        b.iter(|| {
            let arg = 42000;
            let mut acc = 0;
            for i in 1..100000 {
                acc += mul4(arg, i);
            }

            return acc;
        });
    }

}
