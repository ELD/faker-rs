use libfaker::Faker;

fn main() {
    println!("{}", Faker::lorem().paragraphs(3, 5));
}
