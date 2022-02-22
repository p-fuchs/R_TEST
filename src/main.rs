#[macro_use] extern crate prettytable;
mod settings;
mod interface;
mod testing;

fn main() {
    let mut x = settings::Options::new();
    interface::invoke(&mut x);
    x.save();
}
