fn main() {
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}" , value);

    value = !value;
    println!("value is {:08b}" , value);

    value = value & 0b1111_0111;
    println!("value is {:08b}" , value);
    println!("bit 6 is {}", value & 0b0100_00000);

    value = value | 0b0010_0000;
    println!("value is {:08b}" , value);

    value = value ^ 0b0001_0000;
    println!("value is {:08b}" , value);

    value = value << 4;
    println!("value is {:08b}" , value);

    value = value << 2;
    println!("value is {:08b}" , value);
}



// my hello world program

 /*
 fn main() {
    println!("Hello, world!");
}
 
*/
//declaring variables
/*
fn main() {
    let mut x = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);
}
 */

    
//Integer data types
/*
fn main() {
    let mut x: u8 = 255;
    x = x +1 ;
    println!("x is {}", x);
    
} 
 */

//floating point data types
/*
fn main(){
    let x: f32 = 10.123456789654321;
    println!(" x is {}",x);
}
    Airthmetic operations
fn main(){
    let a = 10;
    let b = 3.0;
    let c = a as f64 / (b + 1.0);
    println!("c is {}", c);
}

    Formatting print statements
fn main(){
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    println!("c is {:08.3}\na is {1}\nonce again, c is {0}", c, a);
}

 */