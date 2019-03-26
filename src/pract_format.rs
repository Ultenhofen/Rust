pub fn run() {
    //Print to Console :)
    println!("Hello from land down under lol");

    //Basic Formatting with println
    /*
    Below is the wrong formatting
    println!("1");
    */
    //Here is the correct formatting
    println!(
        "{}",
        1
    );
    println!(
        "{} seems to only be attracted to {}",
        "Max", "lesbians"
    );

    //Positional Arguments
    println!(
        "{0} is only attracted to {1} and {0} has a really attractive {2}",
        "Max","lesbians","mom"
    );

    //Named Arguments
    println!(
        "{name} is a {person}!",
        name = "Jake",
        person = "Chad"
    );

    //Placeholder traits
    println!(
        "Binary: {:b}, Hex: {:x}, Octal: {:o}",
        69, 69, 69
    );
    //Placeholder for debug trait
    println!(
        "{:?}",
        (96,false,"Penis")
    );
    //Basic math can also be done within the println! function thingy
    println!("What is 10 + 10? It is {}!", 10 + 10);
}
