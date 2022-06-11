pub fn run(){
    let mut count:i32 = 0;

    // loop{
    //     count += 1;
    //     println!("Number:{}", count);
    //
    //     if count == 20{
    //         break;
    //     }
    // }
    //
    // While loop(FizzBuzz)
    //
    // while count <= 100{
    //     if count % 15 == 0{
    //         println!("FIZZBUZZ");
    //     }
    //     else if count % 3 == 0{
    //         println!("FIZZ")
    //     }
    //     else if count % 5 == 0{
    //         println!("BUZZ")
    //     }
    //     else{
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }

    // For Range

    for count in 0..100 {
        if count % 15 == 0{
            println!("FIZZBUZZ");
        }
        else if count % 3 == 0{
            println!("FIZZ")
        }
        else if count % 5 == 0{
            println!("BUZZ")
        }
        else{
            println!("{}", count);
        }
    }
}