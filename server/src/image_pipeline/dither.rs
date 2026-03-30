use image::{ImageBuffer, Rgb, RgbImage};

use crate::config::{
    DitherOptions, REFERENCE_PALETTE, SATURATION_BIAS, SATURATION_SCALE, SaturationMode,
    VALUE_BIAS, VALUE_SATURATION_SCALE, VALUE_SCALE,
};

pub fn boost_saturation(image: &RgbImage, saturation_mode: SaturationMode) -> RgbImage {
    if saturation_mode == SaturationMode::Neutral {
        return image.clone();
    }

    let mut output = RgbImage::new(image.width(), image.height());

    for (x, y, pixel) in image.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let (h, s, v) = rgb_to_hsv(r, g, b);
        let (boosted_s, boosted_v) = if s <= f32::EPSILON {
            (0.0, v)
        } else {
            (
                (s * SATURATION_SCALE + SATURATION_BIAS).clamp(0.0, 1.0),
                (v * VALUE_SCALE + s * VALUE_SATURATION_SCALE + VALUE_BIAS).clamp(0.0, 1.0),
            )
        };
        output.put_pixel(x, y, Rgb(hsv_to_rgb(h, boosted_s, boosted_v)));
    }

    output
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    if delta.abs() < f32::EPSILON {
        return (0.0, 0.0, max);
    }

    let s = if max <= f32::EPSILON {
        0.0
    } else {
        delta / max
    };
    let h_base = if (max - r).abs() < f32::EPSILON {
        ((g - b) / delta).rem_euclid(6.0)
    } else if (max - g).abs() < f32::EPSILON {
        ((b - r) / delta) + 2.0
    } else {
        ((r - g) / delta) + 4.0
    };

    (60.0 * h_base, s, max)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    if s <= f32::EPSILON {
        let gray = (v * 255.0).round() as u8;
        return [gray, gray, gray];
    }

    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime.rem_euclid(2.0)) - 1.0).abs());
    let (r1, g1, b1) = match h_prime {
        hp if (0.0..1.0).contains(&hp) => (c, x, 0.0),
        hp if (1.0..2.0).contains(&hp) => (x, c, 0.0),
        hp if (2.0..3.0).contains(&hp) => (0.0, c, x),
        hp if (3.0..4.0).contains(&hp) => (0.0, x, c),
        hp if (4.0..5.0).contains(&hp) => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;

    [
        ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

pub fn apply_reference_dither(image: &RgbImage, options: DitherOptions) -> RgbImage {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut work = image
        .pixels()
        .map(|pixel| pixel.0.map(|channel| channel as f32))
        .collect::<Vec<_>>();
    let mut output = RgbImage::new(image.width(), image.height());

    for y in 0..height {
        let reverse = options.use_zigzag && y % 2 == 1;
        let xs: Box<dyn Iterator<Item = usize>> = if reverse {
            Box::new((0..width).rev())
        } else {
            Box::new(0..width)
        };
        for x in xs {
            let index = y * width + x;
            let old = work[index];
            let clamped = [
                old[0].clamp(0.0, 255.0),
                old[1].clamp(0.0, 255.0),
                old[2].clamp(0.0, 255.0),
            ];
            let replacement = nearest_palette_color(clamped, options);
            output.put_pixel(x as u32, y as u32, Rgb(replacement));
            let error = [
                (clamped[0] - replacement[0] as f32) * options.diffusion_rate,
                (clamped[1] - replacement[1] as f32) * options.diffusion_rate,
                (clamped[2] - replacement[2] as f32) * options.diffusion_rate,
            ];

            if options.use_atkinson {
                if !reverse {
                    diffuse_error(&mut work, width, height, x + 1, y, error, 1.0 / 8.0);
                    diffuse_error(&mut work, width, height, x + 2, y, error, 1.0 / 8.0);
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y + 1, error, 1.0 / 8.0);
                    }
                    diffuse_error(&mut work, width, height, x, y + 1, error, 1.0 / 8.0);
                    diffuse_error(&mut work, width, height, x + 1, y + 1, error, 1.0 / 8.0);
                    diffuse_error(&mut work, width, height, x, y + 2, error, 1.0 / 8.0);
                } else {
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y, error, 1.0 / 8.0);
                    }
                    if x > 1 {
                        diffuse_error(&mut work, width, height, x - 2, y, error, 1.0 / 8.0);
                    }
                    diffuse_error(&mut work, width, height, x + 1, y + 1, error, 1.0 / 8.0);
                    diffuse_error(&mut work, width, height, x, y + 1, error, 1.0 / 8.0);
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y + 1, error, 1.0 / 8.0);
                    }
                    diffuse_error(&mut work, width, height, x, y + 2, error, 1.0 / 8.0);
                }
            } else if !reverse {
                diffuse_error(&mut work, width, height, x + 1, y, error, 7.0 / 16.0);
                if x > 0 {
                    diffuse_error(&mut work, width, height, x - 1, y + 1, error, 3.0 / 16.0);
                }
                diffuse_error(&mut work, width, height, x, y + 1, error, 5.0 / 16.0);
                diffuse_error(&mut work, width, height, x + 1, y + 1, error, 1.0 / 16.0);
            } else {
                if x > 0 {
                    diffuse_error(&mut work, width, height, x - 1, y, error, 7.0 / 16.0);
                }
                diffuse_error(&mut work, width, height, x + 1, y + 1, error, 3.0 / 16.0);
                diffuse_error(&mut work, width, height, x, y + 1, error, 5.0 / 16.0);
                if x > 0 {
                    diffuse_error(&mut work, width, height, x - 1, y + 1, error, 1.0 / 16.0);
                }
            }
        }
    }

    output
}

fn diffuse_error(
    work: &mut [[f32; 3]],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    error: [f32; 3],
    factor: f32,
) {
    if x >= width || y >= height {
        return;
    }

    let pixel = &mut work[y * width + x];
    for channel in 0..3 {
        pixel[channel] += error[channel] * factor;
    }
}

fn nearest_palette_color(pixel: [f32; 3], options: DitherOptions) -> [u8; 3] {
    let mut best = REFERENCE_PALETTE[0];
    let mut best_score = f32::MAX;

    for candidate in REFERENCE_PALETTE {
        let base_distance = if options.use_lab {
            lab_squared_distance(pixel, candidate)
        } else {
            squared_distance(pixel, candidate)
        };
        let mut score = base_distance;
        if is_neutral_candidate(candidate) {
            score += options.neutral_bias;
        } else {
            score += options.chroma_bias;
        }
        score += hue_penalty(pixel, candidate, options.hue_guard);

        if score < best_score {
            best = candidate;
            best_score = score;
        }
    }

    best
}

fn is_neutral_candidate(candidate: [u8; 3]) -> bool {
    matches!(candidate, [0, 0, 0] | [255, 255, 255])
}

fn hue_penalty(pixel: [f32; 3], candidate: [u8; 3], hue_guard: f32) -> f32 {
    if hue_guard <= f32::EPSILON || is_neutral_candidate(candidate) {
        return 0.0;
    }

    let (input_hue, input_sat, _) = rgb_to_hsv(
        pixel[0].clamp(0.0, 255.0) as u8,
        pixel[1].clamp(0.0, 255.0) as u8,
        pixel[2].clamp(0.0, 255.0) as u8,
    );
    let (candidate_hue, candidate_sat, _) = rgb_to_hsv(candidate[0], candidate[1], candidate[2]);

    if input_sat < 0.12 || candidate_sat < 0.12 {
        return 0.0;
    }

    let delta = hue_distance_degrees(input_hue, candidate_hue);
    if delta <= 20.0 {
        return 0.0;
    }

    let normalized = (delta - 20.0) / 160.0;
    normalized * normalized * hue_guard
}

fn hue_distance_degrees(left: f32, right: f32) -> f32 {
    let delta = (left - right).abs().rem_euclid(360.0);
    delta.min(360.0 - delta)
}

fn squared_distance(pixel: [f32; 3], candidate: [u8; 3]) -> f32 {
    let dr = pixel[0].clamp(0.0, 255.0) - candidate[0] as f32;
    let dg = pixel[1].clamp(0.0, 255.0) - candidate[1] as f32;
    let db = pixel[2].clamp(0.0, 255.0) - candidate[2] as f32;
    dr * dr + dg * dg + db * db
}

fn rgb_to_lab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let linearize = |c: f32| -> f32 {
        let c = c.clamp(0.0, 255.0) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let r = linearize(r);
    let g = linearize(g);
    let b = linearize(b);
    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;
    let f = |t: f32| -> f32 {
        if t > 0.008856 {
            t.cbrt()
        } else {
            7.787 * t + 16.0 / 116.0
        }
    };
    let fx = f(x / 0.95047);
    let fy = f(y / 1.0);
    let fz = f(z / 1.08883);
    [116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz)]
}

fn lab_squared_distance(pixel: [f32; 3], candidate: [u8; 3]) -> f32 {
    let pl = rgb_to_lab(pixel[0], pixel[1], pixel[2]);
    let cl = rgb_to_lab(
        candidate[0] as f32,
        candidate[1] as f32,
        candidate[2] as f32,
    );
    let dl = pl[0] - cl[0];
    let da = pl[1] - cl[1];
    let db = pl[2] - cl[2];
    dl * dl + da * da + db * db
}

pub fn rotate_right_90(image: &RgbImage) -> RgbImage {
    let mut rotated = ImageBuffer::new(image.height(), image.width());

    for (x, y, pixel) in image.enumerate_pixels() {
        let new_x = image.height() - 1 - y;
        let new_y = x;
        rotated.put_pixel(new_x, new_y, *pixel);
    }

    rotated
}
