mod settings;


fn main() {
    let x = settings::Options::new();
    x.save();
}
