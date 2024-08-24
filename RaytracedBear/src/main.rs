
mod framebuffer;
mod ray_intersect;
mod sphere;

use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec3;
use std::time::Duration;

use crate::ray_intersect::RayIntersect;
use crate::sphere::Sphere;

use crate::framebuffer::Framebuffer;


pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> u32 {
    let mut closest_distance = f32::INFINITY;
    let mut pixel_color = 0x00FF00; // Color gris claro por defecto


    for object in objects {
        if let Some(distance) = object.ray_intersect(ray_origin, ray_direction) {
            if distance < closest_distance {
                closest_distance = distance;
                pixel_color = object.color; // Asegúrate de que `Sphere` tiene un campo `color`
            }
        }
    }

    pixel_color
}


pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio;

            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();

            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Ant Caster",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Definir las esferas para crear la hormiga
    let objects = [
        // Cabeza
        Sphere {
            center: Vec3::new(0.0, 2.0, -5.0),
            radius: 0.8,
            color: 0x000000, // Color negro
        },
        // Tórax
        Sphere {
            center: Vec3::new(0.0, 0.7, -5.0),
            radius: 0.9,
            color: 0x000000, // Color negro
        },
        // Abdomen
        Sphere {
            center: Vec3::new(0.0, -1.0, -5.0), // Incrementa el valor de y para subir la esfera
            radius: 1.5,
            color: 0x000000, // Color negro
        },
        // Pata izquierda (superior)
        Sphere {
            center: Vec3::new(-1.0, 1.0, -5.0),
            radius: 0.2,
            color: 0x000000, // Color negro
        },
        // Pata derecha (superior)
        Sphere {
            center: Vec3::new(1.0, 1.0, -5.0),
            radius: 0.2,
            color: 0x000000, // Color negro
        },
        // Pata izquierda (media)
        Sphere {
            center: Vec3::new(-1.2, 0.0, -5.0),
            radius: 0.2,
            color: 0x000000, // Color negro
        },
        // Pata derecha (media)
        Sphere {
            center: Vec3::new(1.2, 0.0, -5.0),
            radius: 0.2,
            color: 0x000000, // Color negro
        },
        // Pata izquierda (inferior)
        Sphere {
            center: Vec3::new(-1.0, -1.5, -5.0),
            radius: 0.2,
            color: 0x000000, // Color negro
        },
        // Pata derecha (inferior)
        Sphere {
            center: Vec3::new(1.0, -1.5, -5.0),
            radius: 0.2,
            color: 0x000000, // Color negro
        },
        // Antena izquierda
        Sphere {
            center: Vec3::new(-0.6, 2.8, -5.0),
            radius: 0.15,
            color: 0x000000, // Color negro
        },
        // Antena derecha
        Sphere {
            center: Vec3::new(0.6, 2.8, -5.0),
            radius: 0.15,
            color: 0x000000, // Color negro
        },
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {

        render(&mut framebuffer, &objects);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
