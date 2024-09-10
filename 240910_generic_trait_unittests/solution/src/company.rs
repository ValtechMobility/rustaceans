pub trait CanGetOlder {
    fn get_older (&mut self, by_how_much: i16);
}

pub trait CanSpeak: CanGetOlder {
    fn _speak(&mut self, what: String) {
        println!("{}", what);
    }
}

pub struct Company<T: CanGetOlder> {
    pub ceo: T
}

impl <T> Company<T> 
where T: CanSpeak {
    pub fn celebrate_ceos_birthday(&mut self) {
        self.ceo.get_older(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockPerson {
        age: i16
    }

    impl CanGetOlder for MockPerson {
        fn get_older(&mut self, by_how_much: i16) {
            self.age += by_how_much;
        }
    }

    impl CanSpeak for MockPerson {}

    #[test]
    fn test_celebrate_ceos_birthday() {
        let test_ceo = MockPerson { age: 44 };
        let mut test_company = Company { ceo: test_ceo };
    
        test_company.celebrate_ceos_birthday();
    
        assert_eq!(test_company.ceo.age, 45)
    }
}
