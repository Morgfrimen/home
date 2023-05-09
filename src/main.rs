mod soket;
mod thermometer;

use crate::soket::Soket;
use crate::thermometer::Thermometer;

fn main() {
    println!("Hello, world!");
    let soket = Soket {
        description: String::from("Новое описание розетки"),
        is_enable: true,
        power_now: 500.1,
    };
    let thermometer = Thermometer {
        current_value: 45.88,
    };
    println!("{}", soket.description);
    println!("{}", soket._get_power_now());
    println!("{}", soket.power_now);
    println!("{}", soket.is_enable);
    println!("{}", thermometer._get_current_value())
}
