use error::Error;
use traits::encoder::Encoder;
use template_ids::TemplateId;
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use std::any::Any;

pub struct Dlist {
    template_id: TemplateId,
    length_encoding_length: usize,
    max_length: usize,
    template: Template
}

impl Dlist  {
    pub fn new(template_id: TemplateId, template: Template) -> Result<Dlist, Error> {
        match (template_id) {
            TemplateId::DlistAlpha => {
                Ok(Dlist{
                    template_id,
                    length_encoding_length: 1,
                    max_length: 255,
                    template
                })
            },
            TemplateId::DlistBeta => {
                Ok(Dlist{
                    template_id,
                    length_encoding_length: 2,
                    max_length: 65536,
                    template
                })
            },
            TemplateId::DlistGamma => {
                Ok(Dlist{
                    template_id,
                    length_encoding_length: 3,
                    max_length: 16777215,
                    template
                })
            },
            TemplateId::DlistDelta => {
                Ok(Dlist{
                    template_id,
                    length_encoding_length: 4,
                    max_length: 4294967295,
                    template
                })
            },
            _ => {
                Err(Error::Dlist__new__invalid_template_id)
            }
        }
    }
    pub fn from_jinyang_with_remainder(template_id: TemplateId, jinyang: &[u8]) -> Result<(Dlist, &[u8]), Error> {
        match(template_id) {
            TemplateId::DlistAlpha
            | TemplateId::DlistBeta
            | TemplateId::DlistGamma
            | TemplateId::DlistDelta => {
                let template_and_remainder = Template::from_jinyang_with_remainder(&jinyang);
                let dlist_result = Dlist::new(template_id, template_and_remainder.0);
                match(dlist_result) {
                    Ok(Dlist) => Ok((Dlist, template_and_remainder.1)),
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

impl Encoder for Dlist {
    fn template_id(&self) -> u8 {
        self.template_id as u8
    }
    fn encode_to<'a>(&self, bytes: &[u8], to: &'a mut Vec<u8>) -> Result<(), Error> {
        let mut remainder = &bytes;
        while(remainder.len() > 0) {
            self.
        }
        to.extend_from_slice(&bytes);
        Ok(())
    }
    fn decode_with_remainder<'a>(&self, bytes: &'a [u8]) -> Result<(&'a [u8], &'a [u8]), Error> {
        if bytes.len() < self.length_encoding_length {
            Err(Error::Dlist__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length)
        } else {
            let mut length_encoding = vec![0; 4];
            for i in 0..self.length_encoding_length {
                length_encoding[i] = bytes[i];
            }
            let length = Cursor::new(&length_encoding).read_u32::<LittleEndian>().unwrap() as usize;
            if (bytes.len() < (self.length_encoding_length + length)) {
                Err(Error::Dlist__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length_plus_length)
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
