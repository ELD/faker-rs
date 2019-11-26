pub mod factories;

// Usage:
// Faker::default()::name()
// Faker::default()::address()
// Faker::default()::timestamp()

pub struct Faker;

impl Default for Faker {
    fn default() -> Self {
        Self
    }
}

impl Faker {
    pub fn name(gender: Option<factories::Gender>) -> String {
        // Future? factories::NameFactory::locale(LOCALE)::name(GENDER)
        factories::NameFactory::name(gender)
    }
}
