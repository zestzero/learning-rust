mod _tuple;
mod _static;
mod _extension;
use test_cargo::test_for_loop;

fn main() {
    let mut y = 10;
    println!("ค่าเริ่มต้นของ y: {}", y);
    y = 15;
    println!("ค่าใหม่ของ y: {}", y);
    _tuple::tuple();
    test_for_loop();
    _static::test_static();
    _extension::test_extension();
}
