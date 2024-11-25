#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    age: u8,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "jD0jw@example.com".to_string(),
        phone: "123-456-7890".to_string(),
        age: 25,
    };
    println!("The person's full name is: {}", person.full_name());
    println!("The person's email is: {}", person.email);
    println!("Again: {:#?}", &person);
}