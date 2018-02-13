fn main() {
    println!("Hello, world!");
}

fn mul0(a: i32, b:i32) -> i32 {

    if a == 1{
        return b;
    }

    return mul0(a-1,b) + b;
}

fn mul1(a:i32, b:i32) -> i32 {
    
    if a == 1 {
        return b;
    }

    let result = mul1(half(a),b+b);


    if odd(a) { return result + b };

    return result;

}

fn half(a:i32) -> i32 {
    a >> 1
}

fn odd(a:i32) -> bool {
    a & 0x1 == 1
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_mul0(){
        assert_eq!(42,mul0(2,21));
        assert_eq!(42,mul0(21,2));
    }
    #[test]
    fn test_mul1(){
        assert_eq!(42,mul1(2,21));
        assert_eq!(42,mul1(21,2));
    }
}
