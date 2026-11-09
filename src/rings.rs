use glam::{Vec2, Vec3};

pub fn ring_pixel(sx: f32, sy: f32, sphere_radius: f32) -> Option<(f32, Vec3)> {
    // Plano de anillos inclinado
    let tilt = 0.42_f32;
    let normal = Vec3::new(0.0, tilt.cos(), tilt.sin());

    let ray_origin = Vec3::new(sx, sy, 3.0);
    let ray_direction = Vec3::new(0.0, 0.0, -1.0);

    let denominator = normal.dot(ray_direction);

    if denominator.abs() < 0.0001 {
        return None;
    }

    let t = -normal.dot(ray_origin) / denominator;

    if t <= 0.0 {
        return None;
    }

    let point = ray_origin + ray_direction * t;
    let radius = Vec2::new(point.x, point.z).length();

    let inner = sphere_radius * 1.12;
    let outer = sphere_radius * 1.75;

    if radius < inner || radius > outer {
        return None;
    }

    // Bandas concentricas
    let bands = (radius * 95.0).sin() * 0.5 + 0.5;
    let fine = (radius * 310.0).sin() * 0.5 + 0.5;

    let gap = (radius * 36.0).sin().abs();

    if gap < 0.10 {
        return None;
    }

    let base = Vec3::new(0.50, 0.31, 0.18);
    let bright = Vec3::new(0.95, 0.77, 0.51);

    let color = base.lerp(bright, (bands * 0.7 + fine * 0.3).clamp(0.0, 1.0));

    Some((point.z, color))
}
