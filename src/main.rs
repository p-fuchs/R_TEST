#[macro_use] extern crate prettytable;
mod settings;
mod interface;
mod testing;

fn main() {
    let x = settings::Options::new();
    interface::invoke(&x);
    x.save();
}
