/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits in memory)
Floats: f32, f64
Boolean: true/false
Characters or Chars
Tuples (lists?)
Arrays
*/

//Rust is a statically typed language, which means that it must know the types of all variables at compile time,
//however, the compiler can usually infer what type we want to use based on the value and how we use it.
pub fn run() {
    //Default is "i32"
    let x = 1;

    //Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 45454454545;

    //Find Maximum Value of a type
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    println!("Is the program running? Answer: {}", is_active);

    //Boolean from an expression
    let is_greater = 10 > 5;

    println!("Is greater? Answer: {}", is_greater);

    //Character/Char
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("Here is a standard char, {}, and here is a failed attempt at using a unicode emoji, {}!", a1, face);
    //Loose Ends
    println!("{} {} {}",x,y,z);
}
