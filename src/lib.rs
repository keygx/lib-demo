mod generatetor;

pub fn print_random_number() {
    let n = generatetor::gen_ran();
    println!("Random u8: {}", n);
}
