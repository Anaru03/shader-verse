mod rings;
use crate::noise::{color, hash};
mod clock;
mod noise;
mod shaders;
use glam::Vec3;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::Instant;

const W: usize = 600;
const H: usize = 600;
const PI: f32 = std::f32::consts::PI;

// =============================================
// SHADER 1: AETHERIS - ESTRELLA DE PLASMA
// =============================================

// AETHERIS: estrella de plasma ardiente

// IGNARA: mundo volcanico

// ZEPHYRA: gigante gaseoso con tormentas

fn shade(n: Vec3, selected: usize, time: f32) -> Vec3 {
    match selected {
        0 => shaders::star::star(n, time),
        1 => shaders::rocky::rocky(n, time),
        _ => shaders::gas::gas(n, time),
    }
}

fn to_rgb(c: Vec3) -> u32 {
    let c = c.max(Vec3::ZERO);
    let c = c / (Vec3::ONE + c);
    let c = c.powf(1.0 / 2.2);

    let r = (c.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (c.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (c.z.clamp(0.0, 1.0) * 255.0) as u32;

    (r << 16) | (g << 8) | b
}

fn render(buffer: &mut [u32], selected: usize, time: f32, yaw: f32, pitch: f32) {
    let radius = 0.78_f32;
    let aspect = W as f32 / H as f32;

    for y in 0..H {
        for x in 0..W {
            let sx = (2.0 * (x as f32 + 0.5) / W as f32 - 1.0) * aspect;
            let sy = 1.0 - 2.0 * (y as f32 + 0.5) / H as f32;

            let distance = sx * sx + sy * sy;

            let c = if distance <= radius * radius {
                let z = (radius * radius - distance).sqrt();
                let normal = Vec3::new(sx, sy, z).normalize();

                // Rotacion visual del shader
                let angle = time * 0.65;
                let rotated = Vec3::new(
                    normal.x * angle.cos() + normal.z * angle.sin(),
                    normal.y,
                    -normal.x * angle.sin() + normal.z * angle.cos(),
                );

                shade(
                    glam::Mat3::from_rotation_y(yaw) * glam::Mat3::from_rotation_x(pitch) * rotated,
                    selected,
                    time,
                )
            } else {
                let direction = Vec3::new(sx * 90.0, sy * 90.0, 1.0);
                let stars = hash(direction.floor());

                if stars > 0.9985 {
                    color(0.7, 0.8, 1.0)
                } else {
                    color(0.002, 0.004, 0.015)
                }
            };

            let mut final_color = c;

            if selected == 2 {
                if let Some((ring_z, ring_color)) = rings::ring_pixel(sx, sy, radius) {
                    let sphere_z = if distance <= radius * radius {
                        (radius * radius - distance).sqrt()
                    } else {
                        f32::NEG_INFINITY
                    };

                    if ring_z > sphere_z {
                        final_color = ring_color;
                    }
                }
            }

            buffer[y * W + x] = to_rgb(final_color);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Shader Verse | 1-3: Planets | Arrows: Change | Space: Animate",
        W,
        H,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    let mut buffer = vec![0u32; W * H];
    let clock = clock::ClockMesh::load("assets/models/clock/clock_optimized.obj", 0);

    let sun = clock::ClockMesh::load("assets/models/sun/sun_optimized.obj", 1);
    let moon = clock::ClockMesh::load("assets/models/moon/moon.obj", 2);
    let start = Instant::now();
    let mut selected = 0usize;
    let mut yaw = 0.0_f32;
    let mut pitch = 0.0_f32;
    let mut spin = 0.0_f32;
    let mut animated = false;
    let mut dirty = true;
    let mut frozen_time = 0.0_f32;

    window.set_target_fps(30);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let previous = selected;

        if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
            selected = 0;
        }

        if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
            selected = 1;
        }

        if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
            selected = 2;
        }

        if window.is_key_pressed(Key::Key5, KeyRepeat::No) {
            selected = 4;
        }

        if window.is_key_pressed(Key::Key4, KeyRepeat::No) {
            selected = 3;
        }

        let speed = 0.10;

        if window.is_key_down(Key::Left) {
            yaw -= speed;
            dirty = true;
        }
        if window.is_key_down(Key::Right) {
            yaw += speed;
            dirty = true;
        }
        if window.is_key_down(Key::Up) {
            pitch += speed;
            dirty = true;
        }
        if window.is_key_down(Key::Down) {
            pitch -= speed;
            dirty = true;
        }

        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            yaw = 0.0;
            pitch = 0.0;
            dirty = true;
        }

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            animated = !animated;
            dirty = true;
        }

        if selected != previous {
            dirty = true;
        }

        if animated {
            frozen_time = start.elapsed().as_secs_f32();
            dirty = true;
        }

        if dirty {
            if selected == 1 {
                moon.render(&mut buffer, W, H, frozen_time, yaw, pitch);
            } else if selected == 4 {
                sun.render(&mut buffer, W, H, frozen_time, yaw, pitch);
            } else if selected == 3 {
                clock.render(&mut buffer, W, H, frozen_time, yaw, pitch);
            } else {
                render(&mut buffer, selected, frozen_time, yaw, pitch);
            }
            dirty = false;
        }

        window
            .update_with_buffer(&buffer, W, H)
            .expect("Error al actualizar la ventana");
    }
}
