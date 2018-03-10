#![feature(test)]

extern crate test;

use std::ops::{Sub,Add};


fn main() {
    println!("Hello, world!");
}


fn line_segment_gcm0<T:Ord+Copy+Sub<Output=T>>(mut a: T,mut b: T) -> T {
    while a != b {
        if a<b {
            b = b-a;
        }
        else{
            a = a-b;
        }
    }
    
    a
}

fn line_segment_gcm1<T:Ord+Copy+Sub<Output=T>>(mut a: T,mut b: T) -> T {

        while a != b {
            while a<b{
            b = b-a;
        }
        std::mem::swap(&mut a,&mut b);
    }
    a
}

fn line_segment_gcm2<T:Ord+Copy+Sub<Output=T>>(mut a: T,mut b: T) -> T {

    while a != b {
        b = segment_remainder(a,b);
        std::mem::swap(&mut a,&mut b);
    }
    a
}

fn segment_remainder<T:Ord+Copy+Sub<Output=T>>(a:T, mut b: T) -> T {
    while a<b{
        b = b-a;
    }
    b 
}

fn fast_segment_remainder<T:Ord+Copy+Sub<Output=T>+Add<Output=T>>(a:T,mut b:T) -> T{
    if b <= a { return b }
    if b-a <= a { return b-a }
    
    b = fast_segment_remainder(a+a,b);

    if b <= a { return b }
    
    b-a
}

fn line_segment_gcm3<T:Ord+Copy+Sub<Output=T>+Add<Output=T>>(mut a: T,mut b: T) -> T {

    while a != b {
        b = fast_segment_remainder(a,b);
        std::mem::swap(&mut a,&mut b);
    }
    a
}


#[cfg(test)]
mod tests{
    
    use super::*;

    #[test]
    fn test_line_segment_gcm0(){

        assert_eq!(1,line_segment_gcm0(7,41));
        assert_eq!(4,line_segment_gcm0(12,16));
        assert_eq!(5,line_segment_gcm0(10,25));

    }

    #[test]
    fn test_line_segment_gcm1(){

        assert_eq!(1,line_segment_gcm1(7,41));
        assert_eq!(4,line_segment_gcm1(12,16));
        assert_eq!(5,line_segment_gcm1(10,25));

    }

    #[test]
    fn test_line_segment_gcm2(){

        assert_eq!(1,line_segment_gcm2(7,41));
        assert_eq!(4,line_segment_gcm2(12,16));
        assert_eq!(5,line_segment_gcm2(10,25));

    }

    #[test]
    fn test_line_segment_gcm3(){

        assert_eq!(1,line_segment_gcm3(7,41));
        assert_eq!(4,line_segment_gcm3(12,16));
        assert_eq!(5,line_segment_gcm3(10,25));
    }
    
}


mod benchmarks{
    
    use super::*;
    use test::Bencher;

    const A:u32 = 123456;
    const B:u32 = 556;

    #[bench]
    fn bench_gcm0(bench:&mut Bencher){
        bench.iter(|| line_segment_gcm0(A,B))
    }

    #[bench]
    fn bench_gcm1(bench:&mut Bencher){
        bench.iter(|| line_segment_gcm1(A,B))
    }

    #[bench]
    fn bench_gcm2(bench:&mut Bencher){
        bench.iter(|| line_segment_gcm2(A,B))
    }

    #[bench]
    fn bench_gcm3(bench:&mut Bencher){
        bench.iter(|| line_segment_gcm3(A,B))
    }

}
