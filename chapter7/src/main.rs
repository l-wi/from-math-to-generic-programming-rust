#![feature(test)]

extern crate test;

use std::ops::{Add, Mul};

trait SemiGroup: Clone {}

trait Monoid: SemiGroup {}

trait Group: Monoid {}

trait Op<T> {
    fn op(a: T, b: T) -> T;
}

trait MonoidOp<T: Monoid>: Op<T> {
    fn e() -> T;
}

trait GroupOp<T: Group>: MonoidOp<T> {
    fn inverse(a: T) -> T;
}

struct Plusable;

struct Multiplicable;

/*--- i32 ---*/

impl SemiGroup for i32 {}

impl Monoid for i32 {}

impl Group for i32 {}

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
    fn inverse(a: i32) -> i32 {
        -a
    }
}

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

/*--- f64 ---*/

impl SemiGroup for f64 {}

impl Monoid for f64 {}

impl Group for f64 {}

impl Op<f64> for Multiplicable {
    fn op(a: f64, b: f64) -> f64 {
        a * b
    }
}

impl MonoidOp<f64> for Multiplicable {
    fn e() -> f64 {
        1.0
    }
}

//FIXME assumes nobody hands in 0 or NaNs or infty etc.
impl GroupOp<f64> for Multiplicable {
    fn inverse(a: f64) -> f64 {
        1.0 / a
    }
}

/*--- matrix ---*/

#[derive(Clone, Debug, PartialEq)]
struct Matrix<T: Clone + PartialEq> {
    data: Vec<T>,
    n: usize,
    m: usize,
}

impl<T: PartialEq + Group + Add<Output = T> + Mul<Output = T>> Matrix<T> {
    //FIXME bound checks etc...
    fn new(n: usize, m: usize, data: Vec<T>) -> Matrix<T> {
        assert!(data.len() == n * m, "dimensions do not match");

        Matrix::<T> { n, m, data }
    }

    //FIXME bound checks etc...
    fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.m + col].clone()
    }
}

impl Mul<Matrix<i32>> for Matrix<i32> {
    type Output = Matrix<i32>;

    fn mul(self, rhs: Matrix<i32>) -> Matrix<i32> {
        assert!(rhs.n == self.m, "dimensions do not match");

        let mut result = Vec::<i32>::with_capacity(self.n * rhs.m);

        for i in 0..self.n {
            for j in 0..rhs.m {
                let mut tmp = 0;

                for k in 0..self.m {
                    tmp = tmp + self.get(i, k) * rhs.get(k, j);
                    //println!("{}",tmp);
                }

                result.push(tmp);
            }
        }

        Matrix {
            n: self.n,
            m: rhs.m,
            data: result,
        }
    }
}

impl SemiGroup for Matrix<i32> {}

impl Op<Matrix<i32>> for Multiplicable {
    fn op(a: Matrix<i32>, b: Matrix<i32>) -> Matrix<i32> {
        a * b
    }
}

fn main() {
    //    print!("{}", mul4(15, 14));
}

fn pow_acc_semigroup<T: SemiGroup, U: Op<T>>(mut a: i32, mut b: T, mut r: T) -> T {
    loop {
        if odd(a) {
            r = U::op(b.clone(), r).clone();
            //r += b;

            if a == 1 {
                return r;
            }
        }

        a = half(a);
        //b = b + b;
        b = U::op(b.clone(), b.clone()).clone();
    }
}

fn pow_semigroup<T: SemiGroup, U: Op<T>>(mut a: i32, mut b: T) -> T {
    while !odd(a) {
        b = U::op(b.clone(), b.clone()).clone();
        a = half(a);
    }

    if a == 1 {
        return b;
    }

    pow_acc_semigroup::<T, U>(half(a - 1), U::op(b.clone(), b.clone()), b.clone())
}

fn pow_monoid<T: Monoid, U: MonoidOp<T>>(a: i32, b: T) -> T {
    if a == 0 {
        U::e()
    } else {
        pow_semigroup::<T, U>(a, b)
    }
}

fn pow_group<T: Group, U: GroupOp<T>>(mut a: i32, mut b: T) -> T {
    if a < 0 {
        a = -a;
        b = U::inverse(b);
    }
    pow_monoid::<T, U>(a, b)
}

fn half(a: i32) -> i32 {
    a >> 1
}

fn odd(a: i32) -> bool {
    a & 0x1 == 1
}

fn fibo_recursive(n: i32) -> i32 {
    if n == 1 || n == 0 {
        1
    } else {
        fibo_recursive(n - 1) + fibo_recursive(n - 2)
    }
}

mod benchmarks {

    use super::*;

    use test::Bencher;

    #[bench]
    fn bench_fibo_rec(bench: &mut Bencher) {
        bench.iter(|| {
            let n = test::black_box(31);
            fibo_recursive(n)
        })
    }

    #[bench]
    fn bench_fibo_matrix(bench: &mut Bencher) {
        bench.iter(|| {
            let n = test::black_box(31);

            let unit_matrix = Matrix::new(2, 1, vec![1, 0]);

            let fibo_matrix = Matrix::new(2, 2, vec![1, 1, 1, 0]);

            let result =
                pow_acc_semigroup::<Matrix<i32>, Multiplicable>(n, fibo_matrix, unit_matrix);

            result.get(0, 0)
        })
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_matrix_get() {
        let r = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);

        assert_eq!(1, r.get(0, 0));
        assert_eq!(3, r.get(0, 2));
        assert_eq!(6, r.get(2, 1));
    }

    #[test]
    fn test_matrix_mul() {
        let m1 = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let m2 = Matrix::new(2, 1, vec![4, 3]);

        m1.get(2, 1);

        let r = m1 * m2;

        assert_eq!(10, r.get(0, 0));
        assert_eq!(24, r.get(1, 0));
        assert_eq!(38, r.get(2, 0));
    }

    #[test]
    fn test_matrix_fibo() {
        //the n+1 fibo number
        let n = 31;

        let unit_matrix = Matrix::new(2, 1, vec![1, 0]);

        let fibo_matrix = Matrix::new(2, 2, vec![1, 1, 1, 0]);

        let result = pow_acc_semigroup::<Matrix<i32>, Multiplicable>(n, fibo_matrix, unit_matrix);

        let x = result.get(0, 0);

        assert_eq!(2178309, x);
    }

    #[test]
    fn test_fibo_recursive() {
        let r = fibo_recursive(31);

        assert_eq!(2178309, r);
    }

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
        assert_eq!(0, result);
    }

    #[test]
    fn test_pow_monoid_mul_neutral() {
        //45^0
        let result = pow_monoid::<i32, Multiplicable>(0, 45);
        assert_eq!(1, result);
    }

    #[test]
    fn test_pow_group_negative() {
        let result = pow_group::<i32, Plusable>(-2, -21);
        assert_eq!(42, result);
    }

    #[test]
    fn test_pow_group_mul_f64() {
        let result = pow_group::<f64, Multiplicable>(2, 4.0);
        assert_eq!(16.0, result);
    }

    #[test]
    fn test_pow_group_mul_f64_inverse() {
        let result = pow_group::<f64, Multiplicable>(-3, -4.0);
        assert_eq!(-1.0 / 64.0, result);
    }
}
