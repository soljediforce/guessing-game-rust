use rand::Rng;
use std::io;

fn main() {
    println!("Raad het nummer!");
    let geheime_nummer = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Hier uw nummer invullen a.u.b.");

        let mut gok = String::new();

        io::stdin()
            .read_line(&mut gok)
            .expect("Fout bij het lezen van de gok");

        let gok: u32 = match gok.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Voer alstublieft een geldig nummer in.");
                continue;
            }
        };

        println!("Uw gok is: {gok}");

        match gok.cmp(&geheime_nummer) {
            std::cmp::Ordering::Less => println!("Uw gok is te laag!"),
            std::cmp::Ordering::Greater => println!("Uw gok is te hoog!"),
            std::cmp::Ordering::Equal => {
                println!("Gefeliciteerd! U heeft het juiste nummer geraden!");
                break;
            }
        }
    }
}
