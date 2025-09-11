fn main(){
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;
    let average = ( a as f64 + b  + c as f64) /3.0;


    assert_eq!(average, 45.1);
    println!("Test passed");

}



 /*
my hello world program
 fn main() {
    println!("Hello, world!");
}
 

//declaring variables

fn main() {
    let mut x = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);
}
 

    
//Integer data types

fn main() {
    let mut x: u8 = 255;
    x = x +1 ;
    println!("x is {}", x);
    
} 


//floating point data types

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

Bitwise operations
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

Boolean data type & operations
fn main() {
    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("not a is {}", !a);
    println!("a and b is {}", a && b);
    println!("a or b is {}", a | b);
    println!("a xor b is {}", a ^ b);

    let c = ( a ^ b) && panic!(); //( a ^ b) || (a & b);
    println!("c is {}", c);

Comparison operators
fn main() {
    let a = 1;
    let b = 2;
    println!("a is {} and b is {}", a, b);
    println!("a EQUALS b is {}", a == b);
    println!("a NOT EQUALS b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
    println!("a LESS THAN b is {}", a < b);
}
Char data types
fn main() {
    let letter = 'A';
    let number = '1';
    let finger = "\u{261D}";
    println!("{}\n{}\n{}", letter, number, finger);
}


 */