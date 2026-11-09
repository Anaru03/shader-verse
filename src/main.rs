use glam::Vec3;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::Instant;

const W: usize = 600;
const H: usize = 600;
const PI: f32 = std::f32::consts::PI;

fn color(r: f32, g: f32, b: f32) -> Vec3 {
    Vec3::new(r, g, b)
}

fn mix(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a.lerp(b, t.clamp(0.0, 1.0))
}

fn fract(x: f32) -> f32 {
    x - x.floor()
}

fn hash(p: Vec3) -> f32 {
    fract((p.dot(color(127.1, 311.7, 74.7))).sin() * 43758.5453)
}

fn noise(p: Vec3) -> f32 {
    let i = p.floor();
    let f = p - i;
    let u = f * f * (Vec3::splat(3.0) - f * 2.0);

    let a = hash(i);
    let b = hash(i + Vec3::X);
    let c = hash(i + Vec3::Y);
    let d = hash(i + Vec3::X + Vec3::Y);
    let e = hash(i + Vec3::Z);
    let f1 = hash(i + Vec3::X + Vec3::Z);
    let g = hash(i + Vec3::Y + Vec3::Z);
    let h = hash(i + Vec3::ONE);

    let x1 = a + (b - a) * u.x;
    let x2 = c + (d - c) * u.x;
    let x3 = e + (f1 - e) * u.x;
    let x4 = g + (h - g) * u.x;

    let y1 = x1 + (x2 - x1) * u.y;
    let y2 = x3 + (x4 - x3) * u.y;

    y1 + (y2 - y1) * u.z
}

fn fbm(mut p: Vec3) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;

    for _ in 0..5 {
        value += noise(p) * amplitude;
        p = p * 2.03 + Vec3::splat(13.7);
        amplitude *= 0.5;
    }

    value
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn lighting(n: Vec3) -> f32 {
    let light = color(-0.5, 0.75, 1.0).normalize();
    0.18 + 0.82 * n.dot(light).max(0.0)
}

// =============================================
// SHADER 1: AETHERIS - ESTRELLA DE PLASMA
// =============================================

// AETHERIS: estrella de plasma ardiente
fn star(n: Vec3, time: f32) -> Vec3 {
    let p = n * 5.0;
    let warp = fbm(p * 1.7 + Vec3::splat(time * 0.08));
    let detail = fbm(p * 7.0 + Vec3::new(0.0, time * 0.12, 0.0));

    let filaments = ((n.y * 46.0 + warp * 19.0 + time * 0.7).sin() * 0.5 + 0.5).powf(3.0);

    let cells = fbm(p * 3.2);
    let spots = smoothstep(0.57, 0.72, cells);

    let mut c = mix(color(0.65, 0.025, 0.005), color(1.8, 0.32, 0.015), warp);

    c = mix(c, color(2.5, 1.15, 0.12), filaments * 0.9);
    c = mix(c, color(3.0, 2.2, 0.8), detail * 0.48);
    c = mix(c, color(0.28, 0.015, 0.005), spots * 0.78);

    let rim = (1.0 - n.z.max(0.0)).powf(1.6);
    c += color(1.7, 0.45, 0.04) * rim;

    c
}

// IGNARA: mundo volcanico
fn rocky(n: Vec3, time: f32) -> Vec3 {
    let p = n * 5.5;

    let terrain = fbm(p);
    let mountains = fbm(p * 3.8);
    let fine = fbm(p * 12.0);

    let warped = p + Vec3::splat(terrain * 1.8);
    let fissure = (fbm(warped * 2.4) - 0.49).abs();
    let lava = 1.0 - smoothstep(0.008, 0.045, fissure);

    let hot = fbm(p * 6.0 + Vec3::splat(time * 0.025));

    let mut c = mix(
        color(0.018, 0.020, 0.026),
        color(0.18, 0.075, 0.055),
        terrain,
    );

    c = mix(c, color(0.34, 0.19, 0.13), mountains * 0.38);
    c += color(0.07, 0.055, 0.045) * fine * 0.25;

    let molten = mix(color(2.2, 0.08, 0.005), color(3.0, 0.85, 0.04), hot);

    c *= lighting(n);
    c += molten * lava * 1.3;

    c
}

// ZEPHYRA: gigante gaseoso con tormentas
fn gas(n: Vec3, time: f32) -> Vec3 {
    let p = n * 5.0;

    let warp = fbm(p * 1.8 + Vec3::new(time * 0.025, 0.0, 0.0));
    let fine = fbm(p * 6.0);

    let latitude = n.y * 35.0 + warp * 8.0 + fine * 1.5;
    let bands = latitude.sin() * 0.5 + 0.5;
    let thin = (latitude * 2.4).sin() * 0.5 + 0.5;

    let longitude = n.x.atan2(n.z);
    let dx = (longitude - 0.55) / 0.40;
    let dy = (n.y + 0.18) / 0.16;
    let distance = (dx * dx + dy * dy).sqrt();

    let storm = 1.0 - smoothstep(0.55, 1.1, distance);
    let vortex = ((distance * 18.0 - warp * 8.0).sin() * 0.5 + 0.5) * storm;

    let mut c = mix(color(0.025, 0.13, 0.34), color(0.03, 0.75, 0.82), bands);

    c = mix(c, color(0.48, 0.18, 0.78), thin * 0.35);
    c = mix(c, color(0.72, 0.95, 0.94), fine * 0.25);
    c = mix(c, color(1.2, 0.34, 0.20), storm * 0.75);
    c = mix(c, color(1.5, 0.85, 0.48), vortex * 0.65);

    c * (0.32 + lighting(n) * 0.68)
}

fn shade(n: Vec3, selected: usize, time: f32) -> Vec3 {
    match selected {
        0 => star(n, time),
        1 => rocky(n, time),
        _ => gas(n, time),
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

fn render(buffer: &mut [u32], selected: usize, time: f32) {
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
                let angle = time * 0.12;
                let rotated = Vec3::new(
                    normal.x * angle.cos() + normal.z * angle.sin(),
                    normal.y,
                    -normal.x * angle.sin() + normal.z * angle.cos(),
                );

                shade(rotated, selected, time)
            } else {
                let direction = Vec3::new(sx * 90.0, sy * 90.0, 1.0);
                let stars = hash(direction.floor());

                if stars > 0.9985 {
                    color(0.7, 0.8, 1.0)
                } else {
                    color(0.002, 0.004, 0.015)
                }
            };

            buffer[y * W + x] = to_rgb(c);
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

    let start = Instant::now();
    let mut selected = 0usize;
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

        if window.is_key_pressed(Key::Right, KeyRepeat::No) {
            selected = (selected + 1) % 3;
        }

        if window.is_key_pressed(Key::Left, KeyRepeat::No) {
            selected = (selected + 2) % 3;
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
            render(&mut buffer, selected, frozen_time);
            dirty = false;
        }

        window
            .update_with_buffer(&buffer, W, H)
            .expect("Error al actualizar la ventana");
    }
}
