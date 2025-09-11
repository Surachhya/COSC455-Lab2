fn main(){
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed");

}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
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

Find Average
fn main(){
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;
    let average = ( a as f64 + b  + c as f64) /3.0;


    assert_eq!(average, 45.1);
    println!("Test passed");

}

Arrays
fn main() {
    let mut letters = ['A', 'B', 'C'];
    letters[0] = 'Z';
    let first_letter = letters[0];
    println!("first letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index : usize = numbers.len();
    println!("last number is {}", numbers[index]);
    
}

Multidimensional Arrays
fn main(){
    let parking_lot = [[1,2,3], [4,5,6]];
    let number = parking_lot[1][2];
    println!("number is {}", number);



}
Tuples
fn main() {
    let mut stuff : (u8, f32, char) = (10, 3.14, 'A');
    stuff.0 +=3;
    let first_item = stuff.0;
    println!("first item is {}", first_item);

    let (a, b, c) = stuff;
    println!(" b is {}", b);
}
Function parameters
fn main(){
    say_hello();
    say_hello();
    let x =1;
    let y =2;
    say_the_sum(x,y);
    say_a_number(x as i32);

}
fn say_hello(){
    println!("Hello, world!");
    say_a_number(13);
}
fn say_a_number(number: i32){
    println!("the number is {}", number);
}
fn say_the_sum( a: u8, b:u8){
    let sum = a+b;
    println!("the sum is {}", sum);

}
Function return values
fn main(){
    let result = square(13);
    println!("result is {:?}", result);

}
fn square(x: i32) -> (i32, i32){
    println!("squaring {}", x);
    return (x, x * x);
    println!("End of function");

} 
 */