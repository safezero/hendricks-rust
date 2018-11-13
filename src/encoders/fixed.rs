use error::Error;
use traits::encoder::Encoder;
use template_ids::TemplateId;
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;

pub struct Fixed {
    template_id: TemplateId,
    length: usize,
}

impl Fixed  {
    pub fn new(template_id: TemplateId, length: usize) -> Result<Fixed, Error> {

        let mut optionalError: Option<Error> = None;

        match (template_id) {
            TemplateId::FixedAlpha => {
                if (length < 1) {
                    optionalError = Some(Error::fixed__new__alpha__length_too_small);
                }else if (length > 256) {
                    optionalError = Some(Error::fixed__new__alpha__length_too_big);
                }
            },
            TemplateId::FixedBeta => {
                if (length < 257) {
                    optionalError = Some(Error::fixed__new__beta__length_too_small);
                }
                if (length > 65792) {
                    optionalError = Some(Error::fixed__new__beta__length_too_big);
                }
            },
            _ => {
                optionalError = Some(Error::fixed__new__invalid_template_id);
            }
        }

        if (optionalError == None) {
            Ok(Fixed{template_id, length})
        } else {
            Err(optionalError.unwrap())
        }
    }
    pub fn from_jinyang_with_remainder(template_id: TemplateId, jinyang: &[u8]) -> Result<(Fixed, &[u8]), Error> {
        match(template_id) {
            TemplateId::FixedAlpha => {
                Ok((
                    Fixed::new(
                        TemplateId::FixedAlpha,
                        (&jinyang[0] + 1) as usize
                    ).unwrap(),
                    &jinyang[1..]
                ))
            },
            TemplateId::FixedBeta => {
                Ok((
                    Fixed::new(
                        TemplateId::FixedAlpha,
                        Cursor::new(&jinyang[0..2]).read_u16::<LittleEndian>().unwrap() as usize
                    ).unwrap(),
                    &jinyang[2..]
                ))
            },
            _ => {
                panic!();
            }
        }
    }
    fn length(&self) -> usize {
        self.length
    }
}

impl Encoder for Fixed {
    fn template_id(&self) -> u8 {
        self.template_id as u8
    }
    fn encode_to<'a>(&self, bytes: &[u8], to: &'a mut Vec<u8>) -> Result<(), Error> {
        if bytes.len() != self.length {
            Err(Error::fixed__encode_to__bytes_length_should_match_self_length)
        } else {
            to.extend_from_slice(&bytes);
            Ok(())
        }
    }
    fn decode_with_remainder<'a>(&self, bytes: &'a [u8]) -> Result<(&'a [u8], &'a [u8]), Error> {
        if bytes.len() < self.length {
            Err(Error::fixed__decode_with_remainder__bytes_length_should_be_gte_self_length)
        } else {
            Ok((
                &bytes[0..self.length],
                &bytes[self.length..bytes.len()]
            ))
        }
    }
    fn export_jinyang_to<'a>(&self, to: &'a mut Vec<u8>) {
        match(self.template_id) {
            TemplateId::FixedAlpha => {
                to.push((self.length - 1) as u8);
            },
            TemplateId::FixedBeta => {
                let mut length_encoding = Vec::new();
                length_encoding.write_u16::<LittleEndian>((self.length - 257) as u16).unwrap();
                to.extend_from_slice(&length_encoding[..]);
            },
            _ => panic!()
        }
    }
}
