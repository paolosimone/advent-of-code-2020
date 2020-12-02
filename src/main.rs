use days::Advent;

mod days;

fn main() {
    Advent::new()
        .days
        .iter()
        .for_each(|day| println!("{}", day))
}
