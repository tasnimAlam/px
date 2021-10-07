use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value: f32 = args[1].parse().unwrap();
    let px: f32 = get_px(value);
    let rem: f32 = get_rem(px);

    println!("{:.2}rem", rem);
    println!("{:.2}px", px);
}

fn get_rem(px: f32) -> f32 {
    px / 16.0
}

fn get_px(value: f32) -> f32 {
    value * 0.633
}
