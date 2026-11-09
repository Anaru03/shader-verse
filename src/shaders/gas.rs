use crate::noise::{color, fbm, lighting, mix, smoothstep};
use glam::Vec3;

pub fn gas(n: Vec3, time: f32) -> Vec3 {
    let p = n * 4.0;

    // Capa 1: deformacion de las bandas atmosfericas
    let turbulence = fbm(p * 1.4 + Vec3::new(time * 0.018, 0.0, 0.0));

    // Capa 2: bandas de diferentes grosores
    let latitude = n.y * 26.0 + turbulence * 5.0;
    let broad = latitude.sin() * 0.5 + 0.5;
    let thin = (latitude * 2.7).sin() * 0.5 + 0.5;

    let mut c = mix(color(0.12, 0.055, 0.035), color(0.88, 0.51, 0.24), broad);

    c = mix(c, color(1.0, 0.83, 0.55), thin * 0.38);

    // Capa 3: nubes turbulentas
    let clouds = fbm(p * 3.2);
    c = mix(
        c,
        color(0.95, 0.89, 0.76),
        smoothstep(0.48, 0.75, clouds) * 0.55,
    );

    // Capa 4: tormenta eliptica con estructura interna
    let longitude = n.x.atan2(n.z);
    let dx = (longitude - 0.45) / 0.45;
    let dy = (n.y + 0.15) / 0.19;
    let distance = (dx * dx + dy * dy).sqrt();

    let storm = 1.0 - smoothstep(0.65, 1.05, distance);
    let vortex = (distance * 21.0 - turbulence * 5.0).sin() * 0.5 + 0.5;

    c = mix(c, color(0.72, 0.22, 0.10), storm * 0.75);

    c = mix(c, color(1.0, 0.68, 0.35), storm * vortex * 0.55);

    // Capa 5: iluminacion atmosferica
    let rim = (1.0 - n.z.max(0.0)).powf(3.0);

    c = c * lighting(n);
    c += color(0.35, 0.18, 0.10) * rim * 0.25;

    c
}
