use std::fs;
use std::io::prelude::*;
fn main() {
    let mut speech = String::new();
    speech.push_str("We chooose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard,\n");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");
}


/*use std::fs;

fn main(){
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents are \n{}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }
    let contents = fs::read("planets.txt").unwrap();
    println!("contents are \n{:?}", contents);
}


use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }
    for (index, argument) in env::args().enumerate() {
        println!("Argument {}: {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}

 use std::io;
use rand::prelude::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100.");
    println!("Guess the number:");

    loop {
        let mut guess = String::new();

        
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess : u32 = guess.trim().parse().expect("Failed to parse the guess.");

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou guessed it! The number was {}.", secret_number);
            break;
        }
    }
}


use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter your message: ");
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    print!("number + 1 is {}", number + 1);
}




 fn main(){
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 ;
    let mut min: i32 ;
    let mut mean: f64 ;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for &num in numbers.iter(){
        if num > max {
            max = num;
        }
        else if num < min {
            min = num;
        }
        mean += num as f64;
    }
    mean/= numbers.len() as f64;
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Test passed");
}




Control flow conditional executions
fn main (){
    let x =4;

     if x + 1 !=3{
        println!("x + 1 is not three");
     }
}
multiple conditions
fn main(){
    let x =3;
    let y = 5;

    if x > y{
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }
}
COnditional assignment
fn main(){
    let make_x_odd = true;
    let x;

    if make_x_odd {
        x = 1;
    } else {
        x = 2;
    }
    println!("x is {}", x);
}
Loops
fn main(){
    let mut count = 0;


    let result = loop{
        if count == 10{
            break count * 10;
        
        }
        count +=1;
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("result is {}", result);
}
While loops
fn main() {
    let mut count = 0;
    let letters = ['a', 'b', 'c', 'd'];

    while count < 10 {
        println!("letter is {}", letters[count]);
        count += 1;

    }
}
 For loops
fn main(){
    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate(){
        println!("item {} is {}", index, item);
        if item == 'e'{
            break;
        }

    }
    for number in 0..5{
        println!("number is {}", number);
    }
}
 Nested loops
 fn main(){
    let mut matrix = [[1,2,3], [4,5,6], [7,8,9]];

    for row in matrix.iter_mut(){
        for num in row.iter_mut(){
            *num += 10;
            print!("{}\t ", num);

        
        }
        println!();
    }
}
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


floating point data types

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
Challenge: Chapter4
fn main(){
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed");

}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}
 */
