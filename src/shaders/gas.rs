use crate::noise::*;
use glam::Vec3;

pub fn gas(n: Vec3, time: f32) -> Vec3 {
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
