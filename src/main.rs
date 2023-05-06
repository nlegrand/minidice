use rand::Rng;

fn dice(d: i16) -> i16 {
    rand::thread_rng().gen_range(1..=d)
}

fn main() {
    println!("1d4: {}", dice(4));
    println!("1d6: {}", dice(6));
    println!("1d8: {}", dice(8));
    println!("1d10: {}", dice(10));
    println!("1d12: {}", dice(12));
    println!("1d20: {}", dice(20));
    println!("1d100: {}", dice(100));
}
