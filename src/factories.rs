use rand::Rng;

include!("locales/en_US/names.rs");

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
    pub fn name(gender: Option<Gender>) -> String {
        if let Some(gender) = gender {
            match gender {
                Gender::Male => NameFactory::male_name(),
                Gender::Female => NameFactory::female_name(),
            }
        } else {
            match NameFactory::random_gender() {
                Gender::Male => NameFactory::male_name(),
                Gender::Female => NameFactory::female_name(),
            }
        }
    }

    fn female_name() -> String {
        let name_format = FEMALE_NAME_FORMATS[NameFactory::random_index(FEMALE_NAME_FORMATS.len())];

        let name_segments = name_format.split(' ').count();

        match name_segments {
            2 => format!("{} {}", NameFactory::female_first_name(), NameFactory::last_name()),
            3 => format!("{} {} {}", NameFactory::female_title(), NameFactory::female_first_name(), NameFactory::last_name()),
            4 => format!("{} {} {} {}", NameFactory::female_title(), NameFactory::female_first_name(), NameFactory::last_name(), NameFactory::suffix()),
            _ => unreachable!(),
        }
    }

    fn male_name() -> String {
        let name_format = MALE_NAME_FORMATS[NameFactory::random_index(MALE_NAME_FORMATS.len())];

        let name_segments = name_format.split(' ').count();

        match name_segments {
            2 => format!("{} {}", NameFactory::male_first_name(), NameFactory::last_name()),
            3 => format!("{} {} {}", NameFactory::male_title(), NameFactory::male_first_name(), NameFactory::last_name()),
            4 => format!("{} {} {} {}", NameFactory::male_title(), NameFactory::male_first_name(), NameFactory::last_name(), NameFactory::suffix()),
            _ => unimplemented!()
        }
    }

    fn female_title() -> &'static str {
        TITLE_FEMALE[NameFactory::random_index(TITLE_FEMALE.len())]
    }

    fn female_first_name() -> &'static str {
        FEMALE_FIRST_NAMES[NameFactory::random_index(FEMALE_FIRST_NAMES.len())]
    }

    fn male_title() -> &'static str {
        TITLE_MALE[NameFactory::random_index(TITLE_MALE.len())]
    }

    fn male_first_name() -> &'static str {
        MALE_FIRST_NAMES[NameFactory::random_index(MALE_FIRST_NAMES.len())]
    }

    fn last_name() -> &'static str {
        LAST_NAME[NameFactory::random_index(LAST_NAME.len())]
    }

    fn suffix() -> &'static str {
        SUFFIXES[NameFactory::random_index(SUFFIXES.len())]
    }

    fn random_index(bound: usize) -> usize {
        rand::thread_rng().gen_range(0, bound)
    }

    fn random_gender() -> Gender {
        Gender::from(rand::thread_rng().gen_range(0, 2))
    }
}
