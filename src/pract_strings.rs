//Primitive str = immutable fixed-length string somewhere in memory
//String = growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello");

    //Get Length
    println!(
        "Length: {}",
        hello.len()
    );
    println!();
    println!(
        "{}",
        hello
    );
    println!();
    // Pushes a char onto the string 'hello'
    hello.push(
        ' '
    );
    hello.push(
        'W'
    );
    // Pushes a string to the string 'hello'
    hello.push_str(
        "orld!"
    );
    println!(
        "{}",
        hello
    );
    println!();
    // Utilizing the .capacity() function on a string
    println!(
        "Capacity: {}",
        hello.capacity()
    );
    // Utilizing the .is_empty() function on a string
    println!(
        "Is it empty: {}",
        hello.is_empty()
    );
    // Utilizing the .contains(arg1) function on a string
    println!(
        "Does it contain 'World': {}",
        hello.contains("World")
    );
    //Utilizing the .replace(arg1,arg2) function on a string
    println!(
        "What do you say before ending it all? Answer: {}",
        hello.replace("Hello", "Goodbye")
    );
    // for loop
    for word in hello.split_whitespace(){
        println!(
            "{}",
            word
        );
    }

}
