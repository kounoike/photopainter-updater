use image::RgbImage;

use crate::config::{
    BINARY_FRAME_FLAGS, BINARY_FRAME_MAGIC, BINARY_FRAME_VERSION, BINARY_HEADER_LENGTH,
    EPD_DISPLAY_HEIGHT, EPD_DISPLAY_WIDTH,
};

pub fn encode_binary_frame(image: &RgbImage) -> Vec<u8> {
    let payload = pack_epaper_frame(image);
    let payload_length = payload.len() as u32;
    let checksum = payload_checksum(&payload);
    let mut bytes = Vec::with_capacity(BINARY_HEADER_LENGTH as usize + payload.len());
    bytes.extend_from_slice(&BINARY_FRAME_MAGIC);
    bytes.push(BINARY_FRAME_VERSION);
    bytes.push(BINARY_FRAME_FLAGS);
    bytes.extend_from_slice(&BINARY_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(EPD_DISPLAY_WIDTH as u16).to_le_bytes());
    bytes.extend_from_slice(&(EPD_DISPLAY_HEIGHT as u16).to_le_bytes());
    bytes.extend_from_slice(&payload_length.to_le_bytes());
    bytes.extend_from_slice(&checksum.to_le_bytes());
    bytes.extend_from_slice(&payload);
    bytes
}

fn pack_epaper_frame(image: &RgbImage) -> Vec<u8> {
    let width = EPD_DISPLAY_WIDTH;
    let height = EPD_DISPLAY_HEIGHT;
    let width_bytes = width.div_ceil(2);
    let mut buffer = vec![0x11; width_bytes * height];

    for (x, y, pixel) in image.enumerate_pixels() {
        if x as usize >= width || y as usize >= height {
            continue;
        }
        let color = palette_index_for_rgb(pixel.0);
        let storage_x = width - 1 - x as usize;
        let storage_y = height - 1 - y as usize;
        let addr = storage_x / 2 + storage_y * width_bytes;
        let nibble = color << 4;
        if storage_x % 2 == 0 {
            buffer[addr] = (buffer[addr] & 0x0F) | nibble;
        } else {
            buffer[addr] = (buffer[addr] & 0xF0) | color;
        }
    }

    buffer
}

pub fn palette_index_for_rgb(pixel: [u8; 3]) -> u8 {
    match pixel {
        [0, 0, 0] => 0,
        [255, 255, 255] => 1,
        [255, 255, 0] => 2,
        [255, 0, 0] => 3,
        [0, 0, 255] => 5,
        [0, 255, 0] => 6,
        other => panic!("unexpected transformed color {other:?}"),
    }
}

pub fn payload_checksum(payload: &[u8]) -> u32 {
    payload
        .iter()
        .fold(0u32, |acc, byte| acc.wrapping_add(u32::from(*byte)))
}
