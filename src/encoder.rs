use traits::encoder::Encoder as EncoderTrait;
use encoders::fixed::Fixed;

pub enum Encoder {
    Fixed = Fixed
}

impl EncoderTrait for Encoder {}
