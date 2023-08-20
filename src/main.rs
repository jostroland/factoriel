use std::{io, str::FromStr, fmt::Display, process};



#[derive(Debug)]
enum FactoError {
    ErreurDeConvertion,
}

impl Display for FactoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",match &self {
            FactoError::ErreurDeConvertion => "Erreur de convertion",
        })
    }
} 


#[derive(Debug)]
struct Facto{
    number:u32,
    result:u32,
}

impl Facto {
    fn new(number:u32,result:u32) -> Self{
        Self { 
            number,
            result, 
        }
    } 

    fn calcul_factoriel(number:u32) -> Self{
        let mut result:u32 = 1;
        let mut count:u32 = 1;
        while count <= number {
            result *= count;
            count += 1;
        }

        Facto::new(number, result)
    }
}


impl FromStr for Facto {
    type Err = FactoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = s.to_string();
        match re.trim() {
                 res => res
                .parse::<u32>()
                .map(|res| Facto::calcul_factoriel(res))
                .map_err(|_| FactoError::ErreurDeConvertion),
        }
    }
}




fn main() {
    println!("Entrer un nombre entier");
    let mut entrer_utilisateur = String::new();
    io::stdin()
        .read_line(&mut entrer_utilisateur)
        .expect("Veuillez entrer un numbre entier natuel");

        match entrer_utilisateur.trim().parse::<Facto>() {
            Ok(facto) => println!("Le factoriel de : {}, est égale à: {}", facto.number, facto.result),
            Err(error) => {
                eprintln!("{:?}", error);
                process::exit(1);
            }
        }
}
