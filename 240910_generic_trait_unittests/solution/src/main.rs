mod company;
mod person;

fn main() {
    let mut company = company::Company {
        ceo: person::Person {
            age: 34
        }
    };

    company.celebrate_ceos_birthday();

    println!("The CEO is now {} years old.", company.ceo.age);
}
