use glam::{Vec2, Vec3};

pub struct ClockMesh {
    triangles: Vec<[Vec3; 3]>,
    center: Vec3,
    scale: f32,
    kind: u8,
}

impl ClockMesh {
    pub fn load(path: &str, kind: u8) -> Self {
        let options = tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ignore_points: true,
            ignore_lines: true,
            ..Default::default()
        };

        let (models, _) = tobj::load_obj(path, &options).expect("Error cargando Clock");

        let mut triangles = Vec::new();
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for model in models {
            let mesh = model.mesh;

            for vertex in mesh.positions.chunks_exact(3) {
                let p = Vec3::new(vertex[0], vertex[1], vertex[2]);
                min = min.min(p);
                max = max.max(p);
            }

            for face in mesh.indices.chunks_exact(3) {
                let point = |i: u32| {
                    let k = i as usize * 3;
                    Vec3::new(
                        mesh.positions[k],
                        mesh.positions[k + 1],
                        mesh.positions[k + 2],
                    )
                };

                triangles.push([point(face[0]), point(face[1]), point(face[2])]);
            }
        }

        let center = (min + max) * 0.5;
        let scale = 1.45 / (max - min).max_element();

        println!("Clock cargado: {} triangulos", triangles.len());

        Self {
            triangles,
            center,
            scale,
            kind,
        }
    }

    fn shader(position: Vec3, normal: Vec3) -> Vec3 {
        let noise =
            ((position.x * 19.0).sin() * (position.y * 23.0).cos() * (position.z * 17.0).sin())
                .abs();

        // Capa 1: acero oscuro
        let mut color = Vec3::new(0.30, 0.36, 0.44);

        // Capa 2: bronce
        let bronze = ((position.y * 9.0).sin() * 0.5 + 0.5) * 0.65;
        color = color.lerp(Vec3::new(0.70, 0.37, 0.12), bronze);

        // Capa 3: cobre envejecido
        color = color.lerp(Vec3::new(0.24, 0.46, 0.40), noise * 0.28);

        // Capa 4: energia ambar
        let energy = (position.x * 31.0 + position.z * 27.0)
            .sin()
            .abs()
            .powf(24.0);

        color += Vec3::new(1.0, 0.40, 0.04) * energy * 0.45;

        let light = Vec3::new(-0.5, 0.8, 1.0).normalize();
        let diffuse = normal.dot(light).max(0.0);

        color * (0.65 + diffuse * 0.55)
    }

    fn sun_shader(p: Vec3) -> Vec3 {
        let n = p.normalize_or_zero();

        let a = (n.x * 47.0).sin() * (n.y * 39.0).cos() * (n.z * 51.0).sin();

        let b = (n.x * 103.0 + n.y * 71.0).sin() * (n.z * 87.0).cos();

        let plasma = (a * 0.5 + 0.5).clamp(0.0, 1.0);
        let granules = (b * 0.5 + 0.5).powf(2.0);

        let filaments = (n.y * 38.0 + a * 9.0 + b * 2.0).sin().abs().powf(5.0);

        let spots = if plasma < 0.18 && granules < 0.35 {
            0.70
        } else {
            0.0
        };

        let mut c = Vec3::new(0.85, 0.08, 0.005);
        c = c.lerp(Vec3::new(1.0, 0.43, 0.025), plasma);

        c = c.lerp(Vec3::new(1.0, 0.87, 0.27), granules * 0.70);

        c += Vec3::new(0.9, 0.38, 0.04) * filaments * 0.45;

        c *= 1.0 - spots;

        c
    }

    fn moon_shader(position: Vec3, normal: Vec3) -> Vec3 {
        let p = position * 8.0;

        // Capa 1: roca gris
        let mut c = Vec3::new(0.30, 0.32, 0.35);

        // Capa 2: regiones minerales
        let minerals = ((p.x * 2.7).sin() * (p.y * 3.1).cos() * (p.z * 2.3).sin()) * 0.5 + 0.5;

        c = c.lerp(Vec3::new(0.55, 0.58, 0.62), minerals * 0.55);

        // Capa 3: regiones oscuras
        let dark = ((p.x * 7.0).sin() * (p.y * 5.0).cos()).abs();

        c = c.lerp(Vec3::new(0.14, 0.16, 0.19), dark.powf(4.0) * 0.35);

        // Capa 4: polvo lunar
        let dust = ((p.x * 31.0).sin() * (p.y * 37.0).cos() * (p.z * 29.0).sin()).abs();

        c += Vec3::splat(dust * 0.07);

        // Iluminacion
        let light = Vec3::new(-0.5, 0.8, 1.0).normalize();
        let diffuse = normal.dot(light).abs();

        c * (0.38 + diffuse * 0.62)
    }

    // MECANIC: planeta industrial

    fn mecanic_shader(p: Vec3, normal: Vec3) -> Vec3 {
        let q = p * 5.0;

        // Capa 1: titanio gris acero
        let mut c = Vec3::new(0.55, 0.65, 0.75);

        // Capa 2: variaciones de grafito
        let graphite = ((q.x * 2.3).sin() * (q.z * 2.7).cos()).abs();

        c = c.lerp(Vec3::new(0.32, 0.40, 0.50), graphite * 0.18);

        // Capa 3: detalles plateados
        let silver = ((q.y * 3.1).sin() * (q.x * 2.4).cos()).abs();

        c = c.lerp(Vec3::new(0.76, 0.81, 0.87), silver.powf(3.0) * 0.16);

        // Capa 4: energia naranja tenue
        let energy = ((q.x * 7.0).sin() * (q.y * 8.0).cos() * (q.z * 6.0).sin())
            .abs()
            .powf(18.0);

        c += Vec3::new(0.85, 0.30, 0.06) * energy * 0.10;

        let light = Vec3::new(-0.5, 0.8, 1.0).normalize();
        let diffuse = normal.dot(light).abs();

        c * (0.35 + diffuse * 0.65)
    }

    // BUILDING: planeta ciudad

    fn building_shader(p: Vec3, normal: Vec3) -> Vec3 {
        let q = p * 5.0;

        // Capa 1: concreto gris azulado
        let mut c = Vec3::new(0.55, 0.65, 0.75);

        // Capa 2: acero estructural
        let steel = ((q.x * 2.1).sin() * (q.z * 2.6).cos()).abs();

        c = c.lerp(Vec3::new(0.38, 0.48, 0.58), steel * 0.15);

        // Capa 3: cristales azulados
        let glass = ((q.x * 6.0).sin() * (q.y * 7.0).cos()).abs().powf(8.0);

        c = c.lerp(Vec3::new(0.30, 0.68, 0.86), glass * 0.12);

        // Capa 4: iluminacion ambar discreta
        let amber = ((q.z * 5.0).sin() * (q.y * 6.0).cos()).abs().powf(10.0);

        c += Vec3::new(0.95, 0.48, 0.12) * amber * 0.08;

        let light = Vec3::new(-0.5, 0.8, 1.0).normalize();
        let diffuse = normal.dot(light).abs();

        c * (0.35 + diffuse * 0.65)
    }

    fn diagnostic_shader(normal: Vec3) -> Vec3 {
        let light = Vec3::new(-0.5, 0.8, 1.0).normalize();
        let diffuse = normal.dot(light).abs();

        Vec3::new(0.55, 0.65, 0.75) * (0.35 + diffuse * 0.65)
    }

    // SERAPHIM: gigante gaseoso celestial
    fn seraphim_shader(p: Vec3, normal: Vec3) -> Vec3 {
        let n = p.normalize_or_zero();

        // Capa 1: atmosfera azul profundo
        let mut c = Vec3::new(0.08, 0.22, 0.48);

        // Capa 2: bandas gaseosas
        let bands = (n.y * 32.0 + (n.x * 8.0).sin() * 2.0 + (n.z * 7.0).cos()).sin() * 0.5 + 0.5;

        c = c.lerp(Vec3::new(0.35, 0.78, 0.92), bands * 0.75);

        // Capa 3: turbulencia atmosferica
        let turbulence = ((p.x * 17.0).sin() * (p.y * 13.0).cos() * (p.z * 19.0).sin()).abs();

        c = c.lerp(Vec3::new(0.72, 0.85, 1.0), turbulence * 0.35);

        // Capa 4: energia dorada
        let energy = ((p.x * 11.0).sin() * (p.z * 9.0).cos()).abs().powf(12.0);

        c += Vec3::new(0.85, 0.62, 0.22) * energy * 0.18;

        let light = Vec3::new(-0.5, 0.8, 1.0).normalize();
        let diffuse = normal.dot(light).abs();

        c * (0.42 + diffuse * 0.58)
    }

    pub fn render(
        &self,
        buffer: &mut [u32],
        width: usize,
        height: usize,
        time: f32,
        yaw: f32,
        pitch: f32,
    ) {
        let angle = time * 0.35;

        let rotation = glam::Mat3::from_rotation_y(yaw)
            * glam::Mat3::from_rotation_x(pitch)
            * glam::Mat3::from_rotation_y(angle);

        let mut depth = vec![f32::INFINITY; width * height];

        buffer.fill(0x030610);

        for triangle in &self.triangles {
            let p = triangle.map(|v| rotation * ((v - self.center) * self.scale));

            let normal = (p[1] - p[0]).cross(p[2] - p[0]).normalize_or_zero();

            if normal.length_squared() < 0.000001 {
                continue;
            }

            let screen = p.map(|v| {
                Vec2::new(
                    (v.x + 1.0) * width as f32 * 0.5,
                    (1.0 - v.y) * height as f32 * 0.5,
                )
            });

            let min_x = screen
                .iter()
                .map(|v| v.x)
                .fold(f32::INFINITY, f32::min)
                .floor()
                .max(0.0) as usize;

            let max_x = screen
                .iter()
                .map(|v| v.x)
                .fold(f32::NEG_INFINITY, f32::max)
                .ceil()
                .min((width - 1) as f32) as usize;

            let min_y = screen
                .iter()
                .map(|v| v.y)
                .fold(f32::INFINITY, f32::min)
                .floor()
                .max(0.0) as usize;

            let max_y = screen
                .iter()
                .map(|v| v.y)
                .fold(f32::NEG_INFINITY, f32::max)
                .ceil()
                .min((height - 1) as f32) as usize;

            let edge =
                |a: Vec2, b: Vec2, c: Vec2| (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x);

            let area = edge(screen[0], screen[1], screen[2]);

            if area.abs() < 0.00001 {
                continue;
            }

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let sample = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                    let a = edge(screen[1], screen[2], sample) / area;
                    let b = edge(screen[2], screen[0], sample) / area;
                    let c = 1.0 - a - b;

                    if a < 0.0 || b < 0.0 || c < 0.0 {
                        continue;
                    }

                    let z = a * p[0].z + b * p[1].z + c * p[2].z;
                    let index = y * width + x;

                    if z >= depth[index] {
                        continue;
                    }

                    depth[index] = z;

                    let position = p[0] * a + p[1] * b + p[2] * c;
                    let color = if self.kind == 1 {
                        Self::sun_shader(position)
                    } else if self.kind == 2 {
                        Self::moon_shader(position, normal)
                    } else if self.kind == 3 {
                        Self::mecanic_shader(position, normal)
                    } else if self.kind == 4 {
                        Self::building_shader(position, normal)
                    } else if self.kind == 5 {
                        Self::seraphim_shader(position, normal)
                    } else {
                        Self::shader(position, normal)
                    };

                    let r = (color.x.clamp(0.0, 1.0) * 255.0) as u32;
                    let g = (color.y.clamp(0.0, 1.0) * 255.0) as u32;
                    let b = (color.z.clamp(0.0, 1.0) * 255.0) as u32;

                    buffer[index] = (r << 16) | (g << 8) | b;
                }
            }
        }
    }
}
