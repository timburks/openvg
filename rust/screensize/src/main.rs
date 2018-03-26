#[link(name = "shapes")]
extern {
    fn init(x: &u32, y: &u32);
    fn finish();
}

fn main() {
    let x : u32 = 0;
    let y : u32 = 0;
    unsafe { init(&x, &y); }
    println!("{}!", "Hello, world");
    println!("{} {}", x, y);
    unsafe { finish(); }
}
