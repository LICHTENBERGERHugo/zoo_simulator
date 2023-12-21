#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Espece {
    Chat,
    Chien,
    Oiseau,
    Poisson,
}

#[derive(Debug)]
#[allow(dead_code)]

struct Animal {
    nom: String,
    espece: Espece,
    age: i32,
}

impl Animal {
    fn afficher_details(&self) {
        println!("{} est un {:?} de {} ans", self.nom, self.espece, self.age);
    }
    fn vieillir(&mut self) {
        self.age += 1;
    }
    fn isspecies(&self, espece: &Espece) -> bool {
        self.espece == *espece
    }
}
trait Affichable {
    fn afficher_caracteristiques(&self);
}
impl Affichable for Animal {
    fn afficher_caracteristiques(&self) {
        println!("{} est un {:?} de {} ans", self.nom, self.espece, self.age);
    }
}

// functional programming
fn filter_on_species(animals: &Vec<Animal>, espece: Espece) -> Vec<&Animal> {
    animals
        .iter()
        .filter(|animal| animal.isspecies(&espece))
        .collect()
}
fn tri_par_age(zoo: &mut Vec<Animal>) {
    zoo.sort_by(|a, b| a.age.cmp(&b.age));
}
fn somme_ages(zoo: &Vec<Animal>) -> u8 {
    zoo.iter().fold(0, |acc, animal| acc + animal.age) as u8
}

fn main() {
    let mut animal = Animal {
        nom: String::from("Acacia"),
        espece: Espece::Chat,
        age: 4,
    };
    animal.afficher_details();

    let mut zoo = vec![
        Animal {
            nom: String::from("Acacia"),
            espece: Espece::Chat,
            age: 4,
        },
        Animal {
            nom: String::from("Bambou"),
            espece: Espece::Chien,
            age: 2,
        },
        Animal {
            nom: String::from("Cerisier"),
            espece: Espece::Oiseau,
            age: 1,
        },
        Animal {
            nom: String::from("Dahlia"),
            espece: Espece::Poisson,
            age: 3,
        },
    ];
    zoo.push(animal);
    println!("{:?}", filter_on_species(&zoo, Espece::Chat));
    // afficher_caracteristiques(&animal);
    // println!("{:?}", zoo);
    tri_par_age(&mut zoo);
    println!("{:?}", zoo);
    println!("{}", somme_ages(&zoo));   
}
