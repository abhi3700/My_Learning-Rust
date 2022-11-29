/*
    Here, a cricket team of players is maintained with each having a fav. food habits.
    The meal is prepared based on the fav. food habits of each player.
*/

// ======FOOD w fav fruit, veggies=======================
#[derive(Debug)]
struct Food {
    fruits: Vec<String>,
    veggies: Vec<String>,
}

impl Food {
    fn new(fruits: Vec<String>, veggies: Vec<String>) -> Food {
        Food {
            fruits: fruits,
            veggies: veggies,
        }
    }

    fn add_fruit(&mut self, fruit: String) {
        self.fruits.push(fruit);
    }

    fn remove_fav_fruit(&mut self, fruit: String) {
        self.fruits.retain(|x| x != &fruit);
    }

    fn add_veggie(&mut self, veggie: String) {
        self.veggies.push(veggie);
    }

    fn remove_fav_veggie(&mut self, veggie: String) {
        self.veggies.retain(|x| x != &veggie);
    }
}

// =======PERSON having fav food===========================
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    fav_food: Food,
}

impl Person {
    fn new(name: String, age: u8, food: Food) -> Person {
        Person {
            name,
            age,
            fav_food: Food {
                fruits: food.fruits,
                veggies: food.veggies,
            },
        }
    }

    fn replace_fav_food(&mut self, food: Food) {
        self.fav_food = Food {
            fruits: food.fruits,
            veggies: food.veggies,
        };
    }

    fn update_age(&mut self, age: u8) {
        self.age = age;
    }
}

// ======TEAM=======================
#[derive(Debug)]
struct team {
    name: String,
    players: Vec<Person>,
}

impl team {
    fn new(name: String) -> team {
        team {
            name,
            players: vec![],
        }
    }

    fn add_player(&mut self, player: Person) {
        self.players.push(player);
    }

    fn remove_player(&mut self, name: String) {
        self.players.retain(|x| x.name.ne(&name));
    }
}

// main
pub fn run() {
    // create a team
    let mut t1 = team::new("India".to_string());

    // add players with fav food into the team
    t1.add_player(Person::new(
        "Dhoni".to_string(),
        40,
        Food::new(
            vec![
                "apple".to_string(),
                "mango".to_string(),
                "lichu".to_string(),
            ],
            vec![
                "carrot".to_string(),
                "potato".to_string(),
                "lauki".to_string(),
            ],
        ),
    ));
    t1.add_player(Person::new(
        "Virat".to_string(),
        30,
        Food::new(
            vec![
                "apple".to_string(),
                "mango".to_string(),
                "lichu".to_string(),
            ],
            vec![
                "carrot".to_string(),
                "potato".to_string(),
                "lauki".to_string(),
            ],
        ),
    ));
    t1.add_player(Person::new(
        "Rohit".to_string(),
        35,
        Food::new(
            vec![
                "apple".to_string(),
                "mango".to_string(),
                "lichu".to_string(),
            ],
            vec![
                "carrot".to_string(),
                "potato".to_string(),
                "lauki".to_string(),
            ],
        ),
    ));
    t1.add_player(Person::new(
        "Jadeja".to_string(),
        35,
        Food::new(
            vec![
                "apple".to_string(),
                "mango".to_string(),
                "lichu".to_string(),
            ],
            vec![
                "carrot".to_string(),
                "potato".to_string(),
                "lauki".to_string(),
            ],
        ),
    ));
    t1.add_player(Person::new(
        "Bumrah".to_string(),
        30,
        Food::new(
            vec![
                "apple".to_string(),
                "mango".to_string(),
                "lichu".to_string(),
            ],
            vec![
                "carrot".to_string(),
                "potato".to_string(),
                "lauki".to_string(),
            ],
        ),
    ));
    t1.add_player(Person::new(
        "Shami".to_string(),
        30,
        Food::new(
            vec![
                "apple".to_string(),
                "mango".to_string(),
                "lichu".to_string(),
            ],
            vec![
                "carrot".to_string(),
                "potato".to_string(),
                "lauki".to_string(),
            ],
        ),
    ));
    t1.add_player(Person::new(
        "Pant".to_string(),
        25,
        Food::new(
            vec![
                "apple".to_string(),
                "mango".to_string(),
                "lichu".to_string(),
            ],
            vec![
                "carrot".to_string(),
                "potato".to_string(),
                "lauki".to_string(),
            ],
        ),
    ));

    // print the team
    println!("{:?}", t1);

    // remove a player
    t1.remove_player("Pant".to_string());

    println!("===\nAfter removal, the team is: {:?}", t1);

    // add a fav fruit to "Dhoni" in the team due to his good performance
    t1.players
        .iter_mut()
        .find(|x| x.name.eq("Dhoni"))
        .unwrap()
        .fav_food
        .add_fruit("banana".to_string());

    // remove a fav fruit from "Virat" in the team due to his allergy
    t1.players
        .iter_mut()
        .find(|x| x.name.eq("Virat"))
        .unwrap()
        .fav_food
        .remove_fav_fruit("lichu".to_string());

    // replace the fav food of "Jadeja" in the team due to his change in diet
    t1.players
        .iter_mut()
        .find(|x| x.name.eq("Jadeja"))
        .unwrap()
        .replace_fav_food(Food::new(
            vec![
                "pomegranate".to_string(),
                "grapes".to_string(),
                "avocado".to_string(),
            ],
            vec![
                "carrot".to_string(),
                "potato".to_string(),
                "lauki".to_string(),
            ],
        ));

    // update age of "Bumrah" in the team due to his birthday
    t1.players
        .iter_mut()
        .find(|x| x.name.eq("Bumrah"))
        .unwrap()
        .update_age(31);
}

// TODO: write test
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
