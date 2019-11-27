use rand::Rng;

include!("../locales/en_US/lorem.rs");

pub struct LoremFactory;

impl LoremFactory {
    pub fn word(&self) -> &'static str {
        LOREM_WORD_LIST[self.random_index(LOREM_WORD_LIST.len())]
    }

    pub fn sentence(&self, length: Option<usize>) -> String {
        let mut words = Vec::new();

        (0..length.unwrap_or_else(|| self.random_from_range(4, 10)))
            .for_each(|_| words.push(LOREM_WORD_LIST[self.random_index(LOREM_WORD_LIST.len())]));

        let mut sentence = words.join(" ");
        sentence.push_str(".");
        sentence
    }

    pub fn paragraph(&self, length: Option<usize>) -> String {
        let mut sentences = Vec::new();

        (0..length.unwrap_or_else(|| self.random_from_range(3, 8)))
            .for_each(|_| sentences.push(self.sentence(Some(rand::thread_rng().gen_range(4, 8)))));

        sentences.join(" ")
    }

    pub fn paragraphs(&self, min: usize, max: usize) -> String {
        let mut paragraphs = Vec::new();

        (0..self.random_from_range(min, max)).for_each(|_| paragraphs.push(self.paragraph(None)));

        paragraphs.join("\n")
    }

    fn random_index(&self, bound: usize) -> usize {
        rand::thread_rng().gen_range(0, bound)
    }

    fn random_from_range(&self, low: usize, high: usize) -> usize {
        rand::thread_rng().gen_range(low, high)
    }
}
