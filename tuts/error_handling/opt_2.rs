/* 
    Using Option type for one of the struct parameters.
    
    Here, the middle name can go blank for few names. So, in order to consider the blank for middle name,
    used Option as the type
*/

struct Name {
    first_name: String,
    middle_name: Option<String>, // middle_name can be empty
    last_name: String
}
impl Name {
    fn get_full_name(&self) -> String{
        match &self.middle_name {
            Some(i) => format!("{} {} {}", self.first_name, i, self.last_name),
            None => format!("{} {}", self.first_name, self.last_name)
        }
    }
}

pub fn run() {
    let n1 = Name {
        first_name: "Abhijit".to_string(),
        middle_name: None,
        last_name: "Roy".to_string()
    };

    println!("First Full name: {:?}", n1.get_full_name());

    let n2 = Name {
        first_name: "Dayal".to_string(),
        middle_name: Some("Hari".to_string()),
        last_name: "Roy".to_string()
    };
    println!("Second Full name: {:?}", n2.get_full_name());
}