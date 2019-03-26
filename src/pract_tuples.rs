//Tuples group together values of different types
//Max of 12 elements

//Arrays - Fixed list where elements are the same data types. It must have the exact number of values specified

pub fn run() {
    let person: (&str, &str, i8) = ("Max", "Portland", 21);

    // Prints out a phrase with the correct words in the right order
    println!("{} is from {} and is {} years old!", person.0, person.1, person.2);

    let numbers: [i32; 5] = [1,2,3,4,5];

    // Prints the entire array in order
    println!("The entire array: {:?}", numbers);

    // Prints the first value of the array
    println!("First value of the array: {}", numbers[0]);

    // Getting array Length
    println!("Array Length: {}", numbers.len());
}
