pub mod color {
    pub struct Color(pub u8, pub u8, pub u8);

    impl Color {
        pub const WHITE: Color = Color(255, 255, 255);
    }
}

mod values {
    // like extension methods
    use super::color::Color;
    
    // Not public, so it can't be accessed from outside the module.
    impl Color {
        pub fn red() -> Color {
            Color(255, 0, 0)
        }
    }
}

// Re-export the `Color` type
pub use self::color::Color;

pub fn test_extension() {
    // Actual path to the implementing type and impl in the same module.
    color::Color::WHITE;

    // Impl blocks in different modules are still accessed through a path to the type.
    color::Color::red();

    // Re-exported paths to the implementing type also work.
    Color::red();
    let red = Color::red();
    println!("{}, {}, {}", red.0, red.1, red.2);

    // Does not work, because use in `values` is not pub.
    // values::Color::red();
}