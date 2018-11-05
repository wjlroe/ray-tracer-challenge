use canvas::Canvas;
use matrices::Matrix4;
use rays::Ray;
use tuples::Tuple;
use world::World;

pub struct Camera {
    hsize: u32,
    vsize: u32,
    pub field_of_view: f32, // Unused: can we remove this?
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
    transform: Matrix4,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f32) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f32 / vsize as f32;
        let half_width: f32;
        let half_height: f32;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / hsize as f32;
        Self {
            hsize,
            vsize,
            field_of_view,
            pixel_size,
            half_width,
            half_height,
            transform: Matrix4::default(),
        }
    }

    pub fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        let x_offset = (px as f32 + 0.5) * self.pixel_size;
        let y_offset = (py as f32 + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel =
            self.transform.inverse() * Tuple::point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                canvas.write_pixel(x, y, &color);
            }
        }
        canvas
    }
}

#[test]
fn test_constructing_a_camera() {
    use std::f32::consts::PI;

    let camera = Camera::new(160, 120, PI / 2.0);
    assert_eq!(camera.hsize, 160);
    assert_eq!(camera.vsize, 120);
    assert_eq!(camera.field_of_view, PI / 2.0);
    assert_eq!(camera.transform, Matrix4::default());
}

#[test]
fn test_the_pixel_size_for_a_horizontal_canvas() {
    use std::f32::consts::PI;

    let camera = Camera::new(200, 125, PI / 2.0);
    assert_eq!(camera.pixel_size, 0.01);
}

#[test]
fn test_the_pixel_size_for_a_vertical_canvas() {
    use std::f32::consts::PI;

    let camera = Camera::new(125, 200, PI / 2.0);
    assert_eq!(camera.pixel_size, 0.01);
}

#[test]
fn test_construct_a_ray_through_the_center_of_the_canvas() {
    use std::f32::consts::PI;
    use tuples::Tuple;

    let camera = Camera::new(201, 101, PI / 2.0);
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(ray.origin, Tuple::point(0.0, 0.0, 0.0));
    assert_eq!(ray.direction, Tuple::vector(0.0, 0.0, -1.0));
}

#[test]
fn test_construct_a_ray_through_a_corner_of_the_canvas() {
    use std::f32::consts::PI;
    use tuples::Tuple;

    let camera = Camera::new(201, 101, PI / 2.0);
    let ray = camera.ray_for_pixel(0, 0);
    assert_eq!(ray.origin, Tuple::point(0.0, 0.0, 0.0));
    assert_eq!(ray.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn test_construct_a_ray_when_the_camera_is_transformed() {
    use std::f32::consts::PI;
    use tuples::Tuple;

    let mut camera = Camera::new(201, 101, PI / 2.0);
    camera.transform =
        Matrix4::rotation_y(PI / 4.0) * Matrix4::translation(0.0, -2.0, 5.0);
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(ray.origin, Tuple::point(0.0, 2.0, -5.0));
    assert_eq!(
        ray.direction,
        Tuple::vector(2f32.sqrt() / 2.0, 0.0, -2f32.sqrt() / 2.0)
    );
}

#[test]
fn test_rendering_a_world_with_a_camera() {
    use std::f32::consts::PI;
    use transforms::view_transform;

    let world = World::default();
    let mut camera = Camera::new(11, 11, PI / 2.0);
    let from = Tuple::point(0.0, 0.0, -5.0);
    let to = Tuple::point(0.0, 0.0, 0.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    camera.transform = view_transform(from, to, up);
    let image = camera.render(world);
    assert_eq!(
        image.pixel_at(5, 5),
        Some(&Tuple::color(0.38066, 0.47583, 0.2855))
    );
}
