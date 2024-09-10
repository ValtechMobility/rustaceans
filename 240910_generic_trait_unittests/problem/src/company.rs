use crate::person::Person;

pub struct Company {
    pub ceo: Person
}

impl Company {
    pub fn celebrate_ceos_birthday(&mut self) {
        self.ceo.get_older(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_celebrate_ceos_birthday() {
        let test_ceo = Person { age: 44 };
        let mut test_company = Company { ceo: test_ceo };
    
        test_company.celebrate_ceos_birthday();
    
        assert_eq!(test_company.ceo.age, 45)
    }
}
