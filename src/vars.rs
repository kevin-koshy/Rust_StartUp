pub fn run() {
    let name = "Brad";
    let age = 37;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let(my_name, my_age) = ("Brad", 37);
    println!("My name is {} and my age is {}", my_name, my_age);
}