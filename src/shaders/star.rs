use crate::noise::*;
use glam::Vec3;

pub fn star(n: Vec3, time: f32) -> Vec3 {
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
