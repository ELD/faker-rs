use libfaker::Faker;

fn main() {
    let num_names = std::env::args()
        .nth(1)
        .unwrap_or("10".to_string())
        .parse::<usize>()
        .unwrap_or(10);

    for _ in 0..num_names {
        println!("{}", Faker::name().name(None));
    }
}
