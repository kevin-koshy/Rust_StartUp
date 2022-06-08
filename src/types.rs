pub fn run() {
    let x = 1;
    let y = 2.5;
    let z:i64 = 234234234234;
    let is_active:bool = true;

    //Find Max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Get boolean from expression
    let is_greater = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, face));



}