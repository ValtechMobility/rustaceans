use juniper::{EmptyMutation, RootNode};

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}

pub struct Superhero {
    id: i32,
    name: String,
    abilities: Vec<Ability>,
}

pub struct Ability {
    name: String,
    mana_cost: i32,
}

#[juniper::object(description = "Superhero who serves the City")]
impl Ability {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn mana_cost(&self) -> i32 {
        self.mana_cost
    }
}

#[juniper::object(description = "Superhero who serves the city")]
impl Superhero {
    pub fn new(name: String) -> Self {
        Superhero {
            id: 0,
            name,
            abilities: vec![],
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn abilities(&self) -> &Vec<Ability> {
        &self.abilities
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn superheroes() -> Vec<Superhero> {
        vec![
            Superhero {
                id: 1,
                name: "Bertman".to_string(),
                abilities: vec![Ability {
                    name: "sneak".to_string(),
                    mana_cost: 18,
                }],
            },
            Superhero {
                id: 2,
                name: "Krümelmonster".to_string(),
                abilities: vec![Ability {
                    name: "Cookie Inhaler".to_string(),
                    mana_cost: 20,
                }],
            },
            Superhero {
                id: 3,
                name: "Suppenman".to_string(),
                abilities: vec![
                    Ability {
                        name: "X-Ray".to_string(),
                        mana_cost: 2,
                    },
                    Ability {
                        name: "Lazor".to_string(),
                        mana_cost: 4,
                    },
                ],
            },
        ]
    }
}
