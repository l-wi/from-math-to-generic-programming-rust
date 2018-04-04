#![feature(test)]

extern crate test;

use std::ops::{Add, Mul};

trait SemiGroup: Copy {}

trait Monoid: SemiGroup {}

trait Group: Monoid{}

trait Op<T> {
    fn op(a: T, b: T) -> T;
}

trait MonoidOp<T: Monoid>: Op<T> {
    fn e() -> T;
}

trait GroupOp<T: Group>: MonoidOp<T> {
    fn inverse(a:T) -> T;
}

struct Plusable;

impl Op<i32> for Plusable {
    fn op(a: i32, b: i32) -> i32 {
        a + b
    }
}

impl MonoidOp<i32> for Plusable {
    fn e() -> i32 {
        0
    }
}

impl GroupOp<i32> for Plusable {
    fn inverse(a:i32) -> i32{
        -a
    }
}


struct Multiplicable;

impl Op<i32> for Multiplicable {
    fn op(a: i32, b: i32) -> i32 {
        a * b
    }
}

impl MonoidOp<i32> for Multiplicable {
    fn e() -> i32 {
       1 
    }
}

impl SemiGroup for i32 {}

impl Monoid for i32 {}

impl Group for i32 {}

fn main() {
    //    print!("{}", mul4(15, 14));
}

fn pow_acc_semigroup<T: SemiGroup, U: Op<T>>(mut a: i32, mut b: T, mut r: T) -> T {
    loop {
        if odd(a) {
            r = U::op(b, r);
            //r += b;

            if a == 1 {
                return r;
            }
        }

        a = half(a);
        //b = b + b;
        b = U::op(b, b);
    }
}

fn pow_semigroup<T: SemiGroup, U: Op<T>>(mut a: i32, mut b: T) -> T {
    while !odd(a) {
        b = U::op(b, b);
        a = half(a);
    }

    if a == 1 {
        return b;
    }

    pow_acc_semigroup::<T, U>(half(a - 1), U::op(b, b), b)
}

fn pow_monoid<T: Monoid, U: MonoidOp<T>>(mut a: i32, b: T) -> T {
    if a == 0 {
        U::e()
    } else {
        pow_semigroup::<T, U>(a, b)
    }
}

fn pow_group<T: Group, U: GroupOp<T>>(mut a:i32,mut b:T) -> T {
    if a < 0 {
        a = -a;
        b = U::inverse(b);
    }
    pow_monoid::<T,U>(a,b)
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
    fn test_pow_acc_semigroup_mul() {
        let result = pow_acc_semigroup::<i32, Plusable>(2, 21, 0);
        assert_eq!(42, result);
    }

    #[test]
    fn test_pow_acc_semigroup_pow() {
        let result = pow_acc_semigroup::<i32, Multiplicable>(7, 2, 1);
        assert_eq!(128, result);
    }

    #[test]
    fn test_pow_semigroup() {
        let result = pow_semigroup::<i32, Plusable>(7, 2);
        assert_eq!(14, result);
    }

    #[test]
    fn test_pow_monoid_add_neutral() {
        let result = pow_monoid::<i32, Plusable>(0, 1);
        assert_eq!(0,result);
    }

    #[test]
    fn test_pow_monoid_mul_neutral() {
        //45^0
        let result = pow_monoid::<i32, Multiplicable>(0, 45);
        assert_eq!(1,result);
    }

    #[test]
    fn test_pow_group_negative() {
        let result = pow_group::<i32,Plusable>(-2, -21);
        assert_eq!(42,result);
    }



    /*    #[bench]
    fn bench_mul4(b: &mut Bencher) {
        b.iter(|| {
            let arg = 420;
            let mut acc = 0;
            for i in 1..1000 {
                acc += mul4(arg, i);
            }

            return acc;
        });
    }
*/
}