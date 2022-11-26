/*
    Here, a cricket team of players is maintained with each having a fav. food habits.
    The meal is prepared based on the fav. food habits of each player.
*/

// ======FOOD=======================
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

    fn change_fav_food(&mut self, food: Food) {
        self.fav_food = Food {
            fruits: food.fruits,
            veggies: food.veggies,
        };
    }
}

// ======TEAM=======================
#[derive(Debug)]
struct team {
    name: String,
    players: Vec<Person>,
}

impl team {
    fn new(id: u8, name: String) -> team {
        team {
            id,
            name,
            players: vec![],
        }
    }

    fn add_player(&mut self, player: Person) {
        self.players.push(player);
    }

    fn remove_player(&mut self, player: Person) {
        self.players.retain(|x| x.name.ne(&player.name));
    }
}

// main
pub fn run() {
    // create a team
    let mut t1 = team::new(1, "India".to_string());

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
}
