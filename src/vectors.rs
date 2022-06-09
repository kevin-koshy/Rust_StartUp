use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);


    // Get single Value
    println!("Single Value: {}", numbers[0]);

    println!("{:?}", numbers);

    println!("Vector Length: {}", numbers.len());

    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // let slice: &[i32] = &numbers[1..3];

    // Loop through vector values

    for x in numbers.iter() {
        println!("NUmber:{}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}