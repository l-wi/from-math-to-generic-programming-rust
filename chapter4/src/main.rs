#![feature(test)]


extern crate test;

use std::ops::{Sub};

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

#[cfg(test)]
mod tests{
    
    use super::*;
    use test::Bencher;

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
}
