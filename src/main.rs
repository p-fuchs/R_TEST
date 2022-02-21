mod settings;
mod interface;
mod testing;

fn main() {
    let x = settings::Options::new();
    for i in testing::run_testing(&x) {
        println!("{:?}", i);
    }
    x.save();
}
