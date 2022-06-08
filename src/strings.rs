pub fn run(){
    let  mut hello = String::from("Hello");
    println!("{}", hello);

    //Get Length

    println!("Length: {}", hello.len());
    hello.push('W');
    hello.push_str("orld");

    //Capacity in byts
    println!("Capacity: {}", hello.capacity());

    //check if empty
    println!("is empty: {}", hello.is_empty());

    //Contain substring ?
    println!("Contains world ? {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through String
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //Create string with capacity

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());


    println!("{}", hello);
}