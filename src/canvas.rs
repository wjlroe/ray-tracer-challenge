use std::fmt::Write;
use tuples::Tuple;

fn color_value_to_8bit(value: f32) -> u8 {
    if value > 1.0 {
        255
    } else if value < 0.0 {
        0
    } else {
        (value * 255.0).round() as u8
    }
}

#[test]
fn test_color_value_to_8bit() {
    assert_eq!(color_value_to_8bit(1.1), 255);
    assert_eq!(color_value_to_8bit(-0.5), 0);
    assert_eq!(color_value_to_8bit(0.5), 128);
}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let num = width * height;
        let mut pixels = Vec::with_capacity(num as usize);
        for _ in 0..num {
            pixels.push(Tuple::color(0.0, 0.0, 0.0));
        }
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: &Tuple) {
        let idx = self.coords_to_index(x, y);
        if let Some(pixel) = self.pixels.get_mut(idx) {
            *pixel = color.clone()
        }
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Option<&Tuple> {
        self.pixels.get(self.coords_to_index(x, y))
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::from("P3\n");
        write!(&mut ppm, "{} {}\n", self.width, self.height);
        ppm.push_str("255\n");
        for row in self.pixels.chunks(self.width as usize) {
            for (idx, pixel) in row.iter().enumerate() {
                if idx == 0 {
                    println!("row: {:?}", row);
                }
                if pixel.x != 0.0 || pixel.y != 0.0 || pixel.z != 0.0 {}
                if idx > 0 {
                    ppm.push_str(" ");
                }
                write!(
                    &mut ppm,
                    "{} {} {}",
                    color_value_to_8bit(pixel.x),
                    color_value_to_8bit(pixel.y),
                    color_value_to_8bit(pixel.z)
                );
            }
            ppm.push_str("\n");
        }
        ppm
    }

    pub fn index_to_coords(&self, idx: usize) -> (u32, u32) {
        ((idx as u32 % self.width), self.width / idx as u32)
    }

    fn coords_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}

#[test]
fn test_creating_a_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    let num_pixels = 10 * 20;
    assert_eq!(c.pixels.len(), num_pixels);
    for pixel in c.pixels.iter() {
        assert_eq!(*pixel, Tuple::color(0.0, 0.0, 0.0));
    }
}

#[test]
fn test_writing_pixels_to_a_canvas() {
    let mut c = Canvas::new(10, 20);
    let red = Tuple::color(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, &red);
    assert_eq!(c.pixel_at(2, 3), Some(&red));
    let mut c2 = Canvas::new(5, 3);
    c2.write_pixel(4, 2, &red);
    assert_eq!(c2.pixel_at(4, 2), Some(&red));
}

#[test]
fn test_constructing_the_ppm_header() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();
    let mut expected = String::from("P3\n");
    expected.push_str("5 3\n");
    expected.push_str("255\n");
    assert_string_eq_for_range(ppm, expected, 0, 2);
}

#[test]
fn test_constructing_the_ppm_pixel_data() {
    let mut c = Canvas::new(5, 3);
    let c1 = Tuple::color(1.5, 0.0, 0.0);
    let c2 = Tuple::color(0.0, 0.5, 0.0);
    let c3 = Tuple::color(-0.5, 0.0, 1.0);
    c.write_pixel(0, 0, &c1);
    c.write_pixel(2, 1, &c2);
    c.write_pixel(4, 2, &c3);
    assert_eq!(c.pixel_at(0, 0), Some(&c1));
    assert_eq!(c.pixel_at(2, 1), Some(&c2));
    assert_eq!(c.pixel_at(4, 2), Some(&c3));
    let ppm = c.to_ppm();
    println!("PPM:\n{}\n", ppm);
    let mut expected = String::new();
    expected.push_str("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n");
    expected.push_str("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n");
    expected.push_str("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
    assert_string_eq_for_range(ppm, expected, 3, 5);
}

#[cfg(test)]
fn assert_string_eq_for_range(
    actual: String,
    expected: String,
    from: usize,
    to: usize,
) {
    use std::ops::RangeInclusive;
    let actual_lines = actual.lines().collect::<Vec<_>>();
    let expected_lines = expected.lines().collect::<Vec<_>>();
    let expected_num_of_lines = (to + 1) - from;
    assert!(
        actual_lines.len() >= to,
        "actual's num of lines ({}) less than to ({})",
        actual_lines.len(),
        to
    );
    assert!(
        expected_lines.len() >= expected_num_of_lines,
        "expected's num of lines({}) less than to ({})",
        expected_lines.len(),
        expected_num_of_lines,
    );
    for i in RangeInclusive::new(from, to) {
        assert_eq!(expected_lines[i - from], actual_lines[i]);
    }
}
