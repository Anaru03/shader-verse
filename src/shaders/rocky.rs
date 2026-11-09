use crate::noise::*;
use glam::Vec3;

pub fn rocky(n: Vec3, time: f32) -> Vec3 {
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
