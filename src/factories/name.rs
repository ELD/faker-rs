use rand::Rng;

include!("../locales/en_US/names.rs");

#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

impl From<usize> for Gender {
    fn from(value: usize) -> Self {
        if value == 0 {
            Gender::Male
        } else {
            Gender::Female
        }
    }
}

pub struct NameFactory;

impl NameFactory {
    pub fn name(&self, gender: Option<Gender>) -> String {
        if let Some(gender) = gender {
            match gender {
                Gender::Male => self.male_name(),
                Gender::Female => self.female_name(),
            }
        } else {
            match self.random_gender() {
                Gender::Male => self.male_name(),
                Gender::Female => self.female_name(),
            }
        }
    }

    fn female_name(&self) -> String {
        let name_format = FEMALE_NAME_FORMATS[self.random_index(FEMALE_NAME_FORMATS.len())];

        let name_segments = name_format.split(' ').count();

        match name_segments {
            2 => format!("{} {}", self.female_first_name(), self.last_name()),
            3 => format!(
                "{} {} {}",
                self.female_title(),
                self.female_first_name(),
                self.last_name()
            ),
            4 => format!(
                "{} {} {} {}",
                self.female_title(),
                self.female_first_name(),
                self.last_name(),
                self.suffix()
            ),
            _ => unreachable!(),
        }
    }

    fn male_name(&self) -> String {
        let name_format = MALE_NAME_FORMATS[self.random_index(MALE_NAME_FORMATS.len())];

        let name_segments = name_format.split(' ').count();

        match name_segments {
            2 => format!("{} {}", self.male_first_name(), self.last_name()),
            3 => format!(
                "{} {} {}",
                self.male_title(),
                self.male_first_name(),
                self.last_name()
            ),
            4 => format!(
                "{} {} {} {}",
                self.male_title(),
                self.male_first_name(),
                self.last_name(),
                self.suffix()
            ),
            _ => unimplemented!(),
        }
    }

    fn female_title(&self) -> &'static str {
        TITLE_FEMALE[self.random_index(TITLE_FEMALE.len())]
    }

    fn female_first_name(&self) -> &'static str {
        FEMALE_FIRST_NAMES[self.random_index(FEMALE_FIRST_NAMES.len())]
    }

    fn male_title(&self) -> &'static str {
        TITLE_MALE[self.random_index(TITLE_MALE.len())]
    }

    fn male_first_name(&self) -> &'static str {
        MALE_FIRST_NAMES[self.random_index(MALE_FIRST_NAMES.len())]
    }

    fn last_name(&self) -> &'static str {
        LAST_NAME[self.random_index(LAST_NAME.len())]
    }

    fn suffix(&self) -> &'static str {
        SUFFIXES[self.random_index(SUFFIXES.len())]
    }

    fn random_index(&self, bound: usize) -> usize {
        rand::thread_rng().gen_range(0, bound)
    }

    fn random_gender(&self) -> Gender {
        Gender::from(rand::thread_rng().gen_range(0, 2))
    }
}
