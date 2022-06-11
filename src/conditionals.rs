pub fn run(){
    let age:i32 = 18;
    let check_id:bool = false;
    let knows_age_of_person:bool = true;

    if age >= 21 && check_id || knows_age_of_person{
        println!("Bartende: What do you like to dring ?");
    }
    else if age < 21 && check_id{
        println!("Bartender:Sorry you have to leave !")
    }
    else{
        println!("Bartender: I'll need to see your ID")
    }

    let is_of_age = if age >= 21 {true} else {false};
}

