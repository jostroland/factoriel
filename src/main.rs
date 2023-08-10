use std::io;

fn main() {

    println!("Entrer un nombre entier");
    let mut number_user = String::new();
    io::stdin()
        .read_line(&mut number_user)
        .expect("Veuillez entrer un numbre entier natuel");

    let number_user: u32 = 
        match number_user.trim().parse() {
            Ok(value) => value,
            Err(error) => panic!("{}", error),
        };

    //Calcul du factoriel
    let mut number_facto:u32 = 1;
    let mut count:u32 = 1;
    while count <= number_user {
        number_facto *= count;
        count += 1;
    }
    print!("Le factoriel de {} est {}", number_user, number_facto);
}
