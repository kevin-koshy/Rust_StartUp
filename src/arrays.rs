
use std::mem;
pub fn run(){
 let mut numbers:[i32; 4] = [1, 2, 3, 5];
    numbers[2] = 20;

    // Get single Value
    println!("Single Value: {}", numbers[0]);

    println!("{:?}", numbers);

    println!("Array Length: {}", numbers.len());

    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

   let slice:&[i32] = &numbers[1..3];
   println!("Slice: {:?}", slice);
}