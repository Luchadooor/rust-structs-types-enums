#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn from_email(email: String) -> Self {
        // Split the email into username and domain
        // What is the return value of split_once()?
        // https://doc.rust-lang.org/std/primitive.str.html#method.split_once
        let (username, domain) = email.split_once('@').expect("Invalid email");
        // Concatenate the ""https://"" and domain
        let uri = format!("https://{}", domain);
        Self::new(String::from(username), email, String::from(uri))
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn update_uri(&mut self, uri: String) {
        self.uri = uri;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    let mut fritz = User::from_email(String::from("fritz@google.com"));
    println!("Hello, {:#?}!", fritz);
    fritz.update_uri(String::from("https://fritz.com"));
    println!("Hello2, {:#?}!", fritz);
}
