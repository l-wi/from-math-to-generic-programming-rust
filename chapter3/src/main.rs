#![feature(iterator_step_by)]

use std::slice::IterMut;


fn main() {

    println!("Hello, world!");
}


fn mark_sieve(data:&mut[bool], factor:usize){
    data.iter_mut().step_by(factor).for_each( |k| *k = false);
}

//TODO test that shit
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

#[cfg(test)]
mod tests{
   
    use super::*;
   
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
}

