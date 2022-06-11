pub fn run(){
    greeting("Hello", "Kevin");

    //Bind function values to variable

    let get_sum = add(5, 5);
    println!("Sum:{}", get_sum);

    //closure
    let add_nums = |n1: i32, n2:i32| n1+n2;
    println!("C Sum:{}", add_nums(3,3));
}

fn greeting(greet:&str, name:&str){
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2:i32) -> i32{
    n1 + n2
}

