/***************************************************************************************************************************************************
Somethings to note about variables:
    1. Variables are immutable by default. This means they cannot be changed willy nilly unless you specify to the compiler that you want to change
       the value like a girl changing tampons on a heavy period. Jesus I am fucked up. By change I mean "reassign"
    2. Because you are plebian Max, you feel the need to write down every fucking detail ever. Variables hold primitive data or references to data.
****************************************************************************************************************************************************/
pub fn run() {
    //Basic Variable Work
    //We use 'let' to initialize and assign variables
    let name = "Max";
    let age = 21;
    println!("My name is {} and I am {}", name, age);

    //But if we try to change one of these variables, we will get an error
    //To get around this, we put 'mut' inbetween 'let' and the variable name
    let mut which_tampon = 1;
    println!("She has used {} tampon/s today", which_tampon);
    which_tampon = 2;
    println!("She has used {} tampon/s today", which_tampon);

    //Define Constant
    const ID: i32 = 201;
    println!("ID: {}", ID);

    //Assign multiple variables
    let (my_name, final_exam_score) = ("Max" , 81);
    println!("{} got a {} on his physics final!", my_name, final_exam_score);

}
