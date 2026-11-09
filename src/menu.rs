use font8x8::{BASIC_FONTS, UnicodeFonts};

const BG: u32 = 0x101625;
const FG: u32 = 0xE8ECF8;
const MUTED: u32 = 0x93A4BF;
const ACCENT: u32 = 0x66D9EF;

pub const NAMES: [&str; 6] = ["SUN", "MOON", "ZEPHYRA", "CLOCK", "MECANIC", "BUILDING"];

pub const TYPES: [&str; 6] = [
    "STAR",
    "ROCKY PLANET",
    "GAS GIANT",
    "MECHANICAL PLANET",
    "INDUSTRIAL PLANET",
    "CITY PLANET",
];

fn rect(
    buffer: &mut [u32],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    rw: usize,
    rh: usize,
    color: u32,
) {
    for py in y..(y + rh).min(h) {
        for px in x..(x + rw).min(w) {
            buffer[py * w + px] = color;
        }
    }
}

fn text(buffer: &mut [u32], w: usize, h: usize, x: usize, y: usize, value: &str, color: u32) {
    for (i, ch) in value.chars().enumerate() {
        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (gy, row) in glyph.iter().enumerate() {
                for gx in 0..8 {
                    if row & (1 << gx) != 0 {
                        let px = x + i * 8 + gx;
                        let py = y + gy;
                        if px < w && py < h {
                            buffer[py * w + px] = color;
                        }
                    }
                }
            }
        }
    }
}

pub fn draw(
    buffer: &mut [u32],
    w: usize,
    h: usize,
    selected: usize,
    visible: bool,
    animated: bool,
) {
    if !visible {
        return;
    }

    let panel_w = 230.min(w);
    rect(buffer, w, h, 0, 0, panel_w, h, BG);

    text(buffer, w, h, 16, 18, "SHADER VERSE", ACCENT);
    text(buffer, w, h, 16, 38, "LAB 05 - SHADERS", MUTED);

    rect(
        buffer,
        w,
        h,
        16,
        58,
        panel_w.saturating_sub(32),
        1,
        0x34405A,
    );

    for i in 0..6 {
        let y = 80 + i * 49;
        let color = if selected == i { ACCENT } else { FG };

        if selected == i {
            rect(buffer, w, h, 8, y - 8, 3, 38, ACCENT);
        }

        text(
            buffer,
            w,
            h,
            16,
            y,
            &format!("{}. {}", i + 1, NAMES[i]),
            color,
        );

        text(buffer, w, h, 40, y + 16, TYPES[i], MUTED);
    }

    rect(
        buffer,
        w,
        h,
        16,
        397,
        panel_w.saturating_sub(32),
        1,
        0x34405A,
    );

    text(buffer, w, h, 16, 416, "ARROWS: ROTATE", FG);
    text(buffer, w, h, 16, 438, "SPACE: ANIMATE", FG);
    text(buffer, w, h, 16, 460, "R: RESET", FG);
    text(buffer, w, h, 16, 482, "M: HIDE MENU", FG);
    text(buffer, w, h, 16, 504, "ESC: EXIT", FG);

    text(
        buffer,
        w,
        h,
        16,
        544,
        if animated {
            "ANIMATION: ON"
        } else {
            "ANIMATION: OFF"
        },
        ACCENT,
    );
}
