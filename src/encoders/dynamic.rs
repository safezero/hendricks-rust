use error::Error;
use traits::encoder::Encoder;
use template_ids::TemplateId;
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use std::any::Any;

pub struct Dynamic {
    template_id: TemplateId,
    length_encoding_length: usize,
    max_length: usize
}

impl Dynamic  {
    pub fn new(template_id: TemplateId) -> Result<Dynamic, Error> {
        match (template_id) {
            TemplateId::DynamicAlpha => {
                Ok(Dynamic{
                    template_id,
                    length_encoding_length: 1,
                    max_length: 255
                })
            },
            TemplateId::DynamicBeta => {
                Ok(Dynamic{
                    template_id,
                    length_encoding_length: 2,
                    max_length: 65536
                })
            },
            TemplateId::DynamicGamma => {
                Ok(Dynamic{
                    template_id,
                    length_encoding_length: 3,
                    max_length: 16777215
                })
            },
            TemplateId::DynamicDelta => {
                Ok(Dynamic{
                    template_id,
                    length_encoding_length: 4,
                    max_length: 4294967295
                })
            },
            _ => {
                Err(Error::dynamic__new__invalid_template_id)
            }
        }
    }
    pub fn from_jinyang_with_remainder(template_id: TemplateId, jinyang: &[u8]) -> Result<(Dynamic, &[u8]), Error> {
        match(template_id) {
            TemplateId::DynamicAlpha
            | TemplateId::DynamicBeta
            | TemplateId::DynamicGamma
            | TemplateId::DynamicDelta => {
                let dynamic_result = Dynamic::new(template_id);
                match(dynamic_result) {
                    Ok(dynamic) => Ok((dynamic, jinyang)),
                    Err(error) => Err(error)
                }
            },
            _ => {
                panic!();
            }
        }
    }
    pub fn length_encoding_length(&self) -> usize {
        self.length_encoding_length
    }
}

impl Encoder for Dynamic {
    fn template_id(&self) -> u8 {
        self.template_id as u8
    }
    fn encode_to<'a>(&self, bytes: &[u8], to: &'a mut Vec<u8>) -> Result<(), Error> {
        if bytes.len() > self.max_length {
            Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length)
        } else {
            let mut length_encoding = Vec::new();
            length_encoding.write_u32::<LittleEndian>(bytes.len() as u32);
            to.extend_from_slice(&length_encoding[..self.length_encoding_length]);
            to.extend_from_slice(&bytes);
            Ok(())
        }
    }
    fn decode_with_remainder<'a>(&self, bytes: &'a [u8]) -> Result<(&'a [u8], &'a [u8]), Error> {
        if bytes.len() < self.length_encoding_length {
            Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length)
        } else {
            let mut length_encoding = vec![0; 4];
            for i in 0..self.length_encoding_length {
                length_encoding[i] = bytes[i];
            }
            let length = Cursor::new(&length_encoding).read_u32::<LittleEndian>().unwrap() as usize;
            if (bytes.len() < (self.length_encoding_length + length)) {
                Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length_plus_length)
            } else {
                Ok((
                    &bytes[self.length_encoding_length..self.length_encoding_length + length],
                    &bytes[self.length_encoding_length + length..]
                ))
            }
        }
    }
    fn export_jinyang_to<'a>(&self, to: &'a mut Vec<u8>) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}
