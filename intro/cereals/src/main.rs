#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains = vec![];
    grains.push(Cereal::Rye);
    drop(grains);
    println!("{:?}", grains);
}
