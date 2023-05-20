enum Nums {
    One = 1,
    Two = 2,
    Three = 3,
}

fn main() {
    let thing = Nums::try_from(1).unwrap();
}
