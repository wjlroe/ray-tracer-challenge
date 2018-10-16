use tuples::Tuple;

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

    fn coords_to_index(&self, x: u32, y: u32) -> usize {
        (x * self.width + y) as usize
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
}
