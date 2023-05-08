mod models;
use crate::models::models::Soket;

fn main() {
    println!("Hello, world!");
    let soket = Soket{
        description: String::from("Новое описание розетки"),
        is_enable: true,
        power_now: 500.1
    };
}