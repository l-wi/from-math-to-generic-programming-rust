#![feature(iterator_step_by)]
#![feature(test)]

extern crate test;

fn main() {
  
    let n = 1<<10;
    let limit = n/2;

    let vec = sift2(limit);

    vec.iter().enumerate().for_each( | (i, prime) | {
        if *prime{
            println!("{}",(2*i+3));
        }
    });
}


fn mark_sieve(data:&mut[bool], factor:usize){
    data.iter_mut().step_by(factor).for_each( |k| *k = false);
}

fn sift0(n:usize) -> Vec<bool>{
        let mut vec = vec![true;n];


        let mut i = 0;
        let mut index_square = 3;

        while index_square < n {
            if vec[i] {
                mark_sieve(&mut vec[index_square..],i+i+3);
            }
            i+=1;
            index_square= 2*i*(i+3)+3;
        }

        vec
}

fn sift1(n:usize) -> Vec<bool>{
    let mut vec = vec![true;n];

    let mut i= 0;
    let mut index_square = 3;
    let mut factor = 3;

    while index_square < n {
        if vec[i] {
            mark_sieve(&mut vec[index_square..], factor);
        }
        i+=1;
        factor = i + i + 3;
        index_square = 2*i*(i+3)+3;
    }

    vec
}

fn sift2(n:usize) -> Vec<bool>{
    let mut vec = vec![true;n];

    let mut i= 0;
    let mut index_square = 3;
    let mut factor = 3;

    while index_square < n {
        if vec[i] {
            mark_sieve(&mut vec[index_square..], factor);
        }
        i+=1;
       
        index_square += factor;
        factor += 2; 
        index_square += factor;
    }

    vec
}

fn sift2_u16(n:u16) -> Vec<bool>{
    let mut vec = vec![true;n as usize];

    let mut i:u16= 0;
    let mut index_square:u16 = 3;
    let mut factor:u16 = 3;

    while index_square < n {
        if vec[i as usize] {
            mark_sieve(&mut vec[index_square as usize..], factor as usize);
        }
        i+=1;
       
        index_square += factor;
        factor += 2; 
        index_square += factor;
    }

    vec
}

fn sift2_u64(n:u64) -> Vec<bool>{
    let mut vec = vec![true;n as usize];

    let mut i:u64= 0;
    let mut index_square:u64 = 3;
    let mut factor:u64 = 3;

    while index_square < n {
        if vec[i as usize] {
            mark_sieve(&mut vec[index_square as usize..], factor as usize);
        }
        i+=1;
       
        index_square += factor;
        factor += 2; 
        index_square += factor;
    }

    vec
}

#[cfg(test)]
mod tests{
   
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_sift2(){
        
        
        //last == 21
        let expected = vec!(true,true,true,false,true,true,false,true,true,false,true,false);
        let actual = sift2(12);


        assert_eq!(expected,actual);
        

    }


    
    #[test]
    fn test_sift1(){
        
        
        //last == 21
        let expected = vec!(true,true,true,false,true,true,false,true,true,false,true,false);
        let actual = sift1(12);


        assert_eq!(expected,actual);
        

    }


    #[test]
    fn test_sift0(){
        
        
        //last == 21
        let expected = vec!(true,true,true,false,true,true,false,true,true,false,true,false);
        let actual = sift0(12);


        assert_eq!(expected,actual);
        

    }

    #[test]
    fn test_mark_sieve(){
        let expected = vec!(false,true,false,true,false);
        let mut v = vec!(true,true,true,true,true);

        mark_sieve(&mut v,2);


        assert_eq!(v,expected);
        

    }


    #[bench]
    fn bench_sift0(b: &mut Bencher){
        b.iter( || {
            sift0(1<<10)
        });
    }

    #[bench]
    fn bench_sift1(b: &mut Bencher){
        b.iter( || {
            sift1(1<<10)
        });
    }

    #[bench]
    fn bench_sift2(b: &mut Bencher){
        b.iter( || {
            sift2(1<<10)
        });
    }

    #[bench]
    fn bench_sift2_u16(b: &mut Bencher){
        b.iter( || {
            sift2_u16(1<<10)
        });
    }

    #[bench]
    fn bench_sift2_u64(b: &mut Bencher){
        b.iter( || {
            sift2_u64(1<<10)
        });
    }
}

