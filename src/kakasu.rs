use image::{DynamicImage, GenericImageView, GenericImage, Rgba};
use std::{fmt, error};
use std::string::FromUtf8Error;

#[allow(dead_code)]
/// The number of bits that is used to represent a color field in an RGB pixel.
const NUM_BITS: u8 = 8;

/// The number of least significant bits we will replace with our information in a color field.
const NUM_OF_LSB: u8 = 2;

/// The bit mask used to retrieve the least significant bits of a color field.
const BIT_MASK: u8 = 1 << NUM_OF_LSB;

/// The number of pixels to encode one byte.
const PIXELS_PER_BYTE: u32 = 1;

/// If after getting the encoded byte information stored at a pixel it is equal to this,
/// then that means no information is stored at the pixel.
const EMPTY_PIXEL: &str = "00000000";

#[derive(Debug)]
pub struct KakusuEncodeError;

impl fmt::Display for KakusuEncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KakusuEncodeError: Image not large enough to encode image")
    }
}

impl error::Error for KakusuEncodeError {}

/// Encodes a UTF8 text message in a specified image
///
/// Encoding the message in the specified image is as follows: We split each byte in the message
/// into 4 'parts' (each part contains 2 bits). We then store each 'part' in the 2 least significant
/// bits of each color field in a pixel of the image (RGBA). We use the RGBA format for simplicity,
/// as we can store one byte of information in one pixel. Thus, an image can hold as many characters
/// as there is pixels in the image (though if more complex characters are stored, it might store
/// less).
///
/// ## Arguments
///
/// `msg` - The message to encode.
///
/// `image` - The image to store the message in.
pub fn encode_message(msg: &mut Vec<u8>, image: &DynamicImage) -> Result<DynamicImage, KakusuEncodeError> {
    if !can_encode_image(&image, &msg) {
        return Err(KakusuEncodeError);
    }

    let (width, height) = image.dimensions();
    let mut encoded_img: DynamicImage = DynamicImage::new_rgba8(width, height);

    for x in 0..width {
        for y in 0..height {
            let mut curr_pixel = image.get_pixel(x, y);

            if !msg.is_empty() {
                store_byte_in_pixel(*msg.get(0).unwrap(), &mut curr_pixel);
                msg.remove(0);
            } else {
                // If we have reached the end of our 'message', we just store the pixels normally,
                // but with the 'empty' marker information.
                store_byte_in_pixel(0b00, &mut curr_pixel);
            }

            encoded_img.put_pixel(x, y, curr_pixel);
        }
    }

    Ok(encoded_img)
}

/// Stores a specified byte in a pixel.
fn store_byte_in_pixel(byte: u8, pixel: &mut Rgba<u8>) {
    let mut curr_byte = byte;

    for i in 0..4 {
        let byte_lsb = curr_byte & (BIT_MASK - 1);
        let old_color_lsb = pixel.0[i] & (BIT_MASK - 1);
        pixel.0[i] = pixel.0[i] - old_color_lsb + byte_lsb;
        curr_byte >>= NUM_OF_LSB;
    }
}

/// Are we able to encode the specified message into the specified image?
fn can_encode_image(image: &DynamicImage, msg: &Vec<u8>) -> bool {
    let (width, height) = image.dimensions();
    let total_pixels = width * height;
    let pixels_needed = msg.len() as u32 * PIXELS_PER_BYTE;
    pixels_needed <= total_pixels
}

/// Retrieves the encoded UTF8 message in an image.
pub fn decode_image(image: &DynamicImage) -> Result<String, FromUtf8Error> {
    let (width, height) = image.dimensions();
    let mut buf: Vec<u8> = Vec::new();
    let empty_pixel: String = String::from(EMPTY_PIXEL);

    for x in 0..width {
        for y in 0..height {
            let mut byte_str = String::new();
            let curr_pixel = image.get_pixel(x, y);

            for color_field in 0..4 {
                let lsb = curr_pixel.0[color_field] & (BIT_MASK - 1);
                byte_str = format!("{:02b}{}", lsb, byte_str);
            }

            if byte_str == empty_pixel {
                break;
            }

            buf.push(u8::from_str_radix(byte_str.as_str(), 2).unwrap())
        }
    }

    String::from_utf8(buf)
}
