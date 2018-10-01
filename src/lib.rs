//! This crate provides native rust implementations of
//! image encoders and decoders and basic image manipulation
//! functions.

#![warn(missing_docs)]
#![warn(unused_qualifications)]
#![deny(missing_copy_implementations)]
#![cfg_attr(all(test, feature = "benchmarks"), feature(test))]
// it's a bit of a pain otherwise
#![cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]

extern crate byteorder;
extern crate lzw;
extern crate num_iter;
extern crate num_rational;
extern crate num_traits;
#[cfg(all(test, feature = "benchmarks"))]
extern crate test;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use std::io::Write;

pub use color::ColorType::{self, Gray, GrayA, Palette, RGB, RGBA, BGR, BGRA};

pub use color::{Luma, LumaA, Rgb, Rgba, Bgr, Bgra};

pub use image::{AnimationDecoder,
                GenericImage,
                GenericImageView,
                ImageDecoder,
                ImageDecoderExt,
                ImageError,
                ImageResult,
                MutPixels,
                // Iterators
                Pixels,
                SubImage};

pub use imageops::FilterType::{self, CatmullRom, Gaussian, Lanczos3, Nearest, Triangle};

pub use image::ImageFormat::{self, BMP, GIF, ICO, JPEG, PNG, PNM, WEBP};

pub use image::ImageOutputFormat;

pub use buffer::{ConvertBuffer,
                 GrayAlphaImage,
                 GrayImage,
                 // Image types
                 ImageBuffer,
                 Pixel,
                 RgbImage,
                 RgbaImage};

// Traits
pub use traits::Primitive;

// Opening and loading images
pub use dynimage::{guess_format, load, load_from_memory, load_from_memory_with_format, open,
                   save_buffer};

pub use dynimage::DynamicImage::{self, ImageLuma8, ImageLumaA8, ImageRgb8, ImageRgba8, ImageBgr8, ImageBgra8};

pub use animation::{Frame, Frames};

// Math utils
pub mod math;

// Image processing functions
pub mod imageops;

// Image codecs
#[cfg(feature = "bmp")]
pub mod bmp;
#[cfg(feature = "dxt")]
pub mod dxt;
#[cfg(feature = "gif_codec")]
pub mod gif;
#[cfg(feature = "hdr")]
pub mod hdr;
#[cfg(feature = "ico")]
pub mod ico;
#[cfg(feature = "jpeg")]
pub mod jpeg;
#[cfg(feature = "png_codec")]
pub mod png;
#[cfg(feature = "pnm")]
pub mod pnm;
#[cfg(feature = "tga")]
pub mod tga;
#[cfg(feature = "tiff")]
pub mod tiff;
#[cfg(feature = "webp")]
pub mod webp;

mod animation;
mod buffer;
mod color;
mod dynimage;
mod image;
mod traits;
mod utils;

// Copies data from `src` to `dst`
//
// Panics if the length of `dst` is less than the length of `src`.
#[inline]
fn copy_memory(src: &[u8], mut dst: &mut [u8]) {
    let len_src = src.len();
    assert!(dst.len() >= len_src);
    dst.write_all(src).unwrap();
}
