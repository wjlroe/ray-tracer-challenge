pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Color { red, green, blue }
    }
}

#[test]
fn test_new_color() {
    let color = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(color.red, -0.5);
    assert_eq!(color.green, 0.4);
    assert_eq!(color.blue, 1.7);
}
