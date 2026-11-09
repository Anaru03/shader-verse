use glam::Vec3;

pub fn color(r: f32, g: f32, b: f32) -> Vec3 {
    Vec3::new(r, g, b)
}

pub fn mix(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a.lerp(b, t.clamp(0.0, 1.0))
}

pub fn fract(x: f32) -> f32 {
    x - x.floor()
}

pub fn hash(p: Vec3) -> f32 {
    fract((p.dot(color(127.1, 311.7, 74.7))).sin() * 43758.5453)
}

pub fn noise(p: Vec3) -> f32 {
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

pub fn fbm(mut p: Vec3) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;

    for _ in 0..3 {
        value += noise(p) * amplitude;
        p = p * 2.03 + Vec3::splat(13.7);
        amplitude *= 0.5;
    }

    value
}

pub fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn lighting(n: Vec3) -> f32 {
    let light = color(-0.5, 0.75, 1.0).normalize();
    0.18 + 0.82 * n.dot(light).max(0.0)
}
