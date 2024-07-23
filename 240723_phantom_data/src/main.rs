use std::{collections::HashMap, marker::PhantomData};

fn main() {
    let mut safe = PasswordSafe::new();
    safe.info();

    let mut open_safe = safe.authorize("admin", "password").unwrap(); // Or use match to handle the error
    open_safe.info();
    open_safe.list_passwords();
    open_safe.store_password("benedikt", "123456");
    open_safe.list_passwords();

    let closed_safe = open_safe.close();
    closed_safe.info();
}

// Empty structs don't take up any space (0 bytes)
// They are purley 'cosmetic' and used to differentiate between states
struct Locked;
struct Unlocked;

struct PasswordSafe<State = Locked> {
    data: HashMap<String, String>,
    state: PhantomData<State>,
}

// Implement "shared" logic both states can use
impl<State> PasswordSafe<State> {
    fn info(&self) {
        println!("Password safe version 1.0 --- State: {:?}", self.state)
    }
}

// Implement state-specific logic
impl PasswordSafe<Locked> {
    fn new() -> Self {
        PasswordSafe {
            data: HashMap::new(),
            state: Default::default(),
        }
    }

    fn authorize(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<PasswordSafe<Unlocked>, String> {
        if username == "admin" && password == "password" {
            return Ok(PasswordSafe {
                data: self.data.clone(),
                state: PhantomData,
            });
        }

        Err(String::from("Unauthorized"))
    }
}

impl PasswordSafe<Unlocked> {
    fn list_passwords(&self) {
        if self.data.is_empty() {
            println!("No passwords stored");
            return;
        }
        self.data.iter().for_each(|(k, v)| {
            println!("{}: {}", k, v);
        })
    }

    fn store_password(&mut self, username: &str, password: &str) {
        self.data.insert(username.to_string(), password.to_string());
    }

    fn close(&mut self) -> PasswordSafe<Locked> {
        PasswordSafe {
            data: self.data.clone(),
            state: PhantomData,
        }
    }
}
