pub fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // สามารถ Destructuring ได้
    println!("ค่าจาก tuple: {}, {}, {}", x, y, z);
}
