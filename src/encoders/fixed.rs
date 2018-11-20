use error::Error;
use traits::encoder::Encoder;
use template_ids::TemplateId;
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use std::any::Any;
use nest::Nest;

pub struct Fixed {
    template_id: TemplateId,
    length: usize,
}

impl Fixed  {
    pub fn new(template_id: TemplateId, length: usize) -> Result<Fixed, Error> {
        let mut optional_error: Option<Error> = None;
        match (template_id) {
            TemplateId::FixedAlpha => {
                if (length < 1) {
                    optional_error = Some(Error::fixed__new__alpha__length_too_small);
                }else if (length > 256) {
                    optional_error = Some(Error::fixed__new__alpha__length_too_big);
                }
            },
            TemplateId::FixedBeta => {
                if (length < 257) {
                    optional_error = Some(Error::fixed__new__beta__length_too_small);
                }
                if (length > 65792) {
                    optional_error = Some(Error::fixed__new__beta__length_too_big);
                }
            },
            _ => {
                optional_error = Some(Error::fixed__new__invalid_template_id);
            }
        }

        if (optional_error == None) {
            Ok(Fixed{template_id, length})
        } else {
            Err(optional_error.unwrap())
        }
    }
    pub fn from_jinyang_with_remainder(template_id: TemplateId, jinyang: &[u8]) -> Result<(Fixed, &[u8]), Error> {
        match(template_id) {
            TemplateId::FixedAlpha => {
                Ok((
                    Fixed::new(
                        template_id,
                        (jinyang[0] as usize) + 1
                    ).unwrap(),
                    &jinyang[1..]
                ))
            },
            TemplateId::FixedBeta => {
                Ok((
                    Fixed::new(
                        template_id,
                        Cursor::new(&jinyang[0..2]).read_u16::<LittleEndian>().unwrap() as usize + 257
                    ).unwrap(),
                    &jinyang[2..]
                ))
            },
            _ => {
                panic!();
            }
        }
    }
    pub fn length(&self) -> usize {
        self.length
    }

}

impl Encoder for Fixed {
    fn template_id(&self) -> u8 {
        self.template_id as u8
    }
    fn encode_to<'a>(&self, nest: &Nest, to: &'a mut Vec<u8>) -> Result<(), Error> {
        let bytes = nest.bytes();
        if bytes.len() != self.length {
            Err(Error::fixed__encode_to__bytes_length_should_match_self_length)
        } else {
            to.extend_from_slice(&bytes);
            Ok(())
        }
    }
    fn decode_with_remainder<'a>(&self, bytes: &'a [u8]) -> Result<(Nest<'a>, &'a [u8]), Error> {
        if bytes.len() < self.length {
            Err(Error::fixed__decode_with_remainder__bytes_length_should_be_gte_self_length)
        } else {
            Ok((
                Nest::Bytes(&bytes[0..self.length]),
                &bytes[self.length..]
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
                length_encoding.write_u16::<LittleEndian>((self.length - 257) as u16);
                to.extend_from_slice(&length_encoding[..]);
            },
            _ => panic!()
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod fixed {

    use template_ids::TemplateId;
    use template::Template;
    use encoders::fixed::Fixed;
    use traits::encoder::Encoder;
    use error::Error;
    use nest::Nest;

    fn create_fixed_template_then<F>(
        template_id: TemplateId,
        length: usize,
        then: F
    ) where F: Fn(Template) {
        match template_id {
            TemplateId::FixedAlpha | TemplateId::FixedBeta => {
                let fixedEncoderResult = Fixed::new(template_id, length);
                match fixedEncoderResult {
                    Ok(fixedEncoder) => {
                        then(Template::new(Box::new(fixedEncoder)));
                    },
                    Err(err) => {
                        panic!();
                    }
                }
            },
            _ => {
                panic!();
            }
        };
    }

    #[test]
    fn should_throw_error_when_creating_fixed_template_with_dynamic_template_id() {
        assert_eq!(
            Fixed::new(TemplateId::DynamicAlpha, 1).err(),
            Some(Error::fixed__new__invalid_template_id)
        );
    }

    #[test]
    fn should_create_fixed_alpha_with_min() {
        assert_eq!(
            Fixed::new(TemplateId::FixedAlpha, 1).err(),
            None
        );
    }

    #[test]
    fn should_create_fixed_alpha_with_max() {
        assert_eq!(
            Fixed::new(TemplateId::FixedAlpha, 256).err(),
            None
        );
    }

    #[test]
    fn should_create_fixed_beta_with_min() {
        assert_eq!(
            Fixed::new(TemplateId::FixedBeta, 257).err(),
            None
        );
    }

    #[test]
    fn should_create_fixed_beta_with_max() {
        assert_eq!(
            Fixed::new(TemplateId::FixedBeta, 65792).err(),
            None
        );
    }

    #[test]
    fn should_error_when_too_small_alpha() {
        assert_eq!(
            Fixed::new(TemplateId::FixedAlpha, 0).err(),
            Some(Error::fixed__new__alpha__length_too_small)
        );
    }

    #[test]
    fn should_error_when_too_big_alpha() {
        assert_eq!(
            Fixed::new(TemplateId::FixedAlpha, 257).err(),
            Some(Error::fixed__new__alpha__length_too_big)
        );
    }

    #[test]
    fn should_error_when_too_small_beta() {
        assert_eq!(
            Fixed::new(TemplateId::FixedBeta, 256).err(),
            Some(Error::fixed__new__beta__length_too_small)
        );
    }

    #[test]
    fn should_error_when_too_big_beta() {
        assert_eq!(
            Fixed::new(TemplateId::FixedBeta, 65793).err(),
            Some(Error::fixed__new__beta__length_too_big)
        );
    }

    #[test]
    fn should_encode_alpha() {
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(template.encode(&Nest::Bytes(&[1])), Ok(vec![1]))
        });
        create_fixed_template_then(TemplateId::FixedAlpha, 256, |template| {
            assert_eq!(template.encode(&Nest::Bytes(&[1; 256])), Ok(vec![1; 256]))
        });
    }

    #[test]
    fn should_encode_beta() {
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(template.encode(&Nest::Bytes(&[1; 257])), Ok(vec![1; 257]))
        });
        create_fixed_template_then(TemplateId::FixedBeta, 65792, |template| {
            assert_eq!(template.encode(&Nest::Bytes(&[1; 65792])), Ok(vec![1; 65792]))
        });
    }

    #[test]
    fn should_decode_alpha() {
        let encoding1 = [1];
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(
                template.decode(&encoding1),
                Ok(Nest::Bytes(&[1])
            ))
        });
        let encoding2 = [1; 256];
        create_fixed_template_then(TemplateId::FixedAlpha, 256, |template| {
            assert_eq!(
                template.decode(&encoding2),
                Ok(Nest::Bytes(&encoding2[..]))
            )
        });
    }

    #[test]
    fn should_decode_beta() {
        let encoding1 = [1; 257];
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(
                template.decode(&encoding1),
                Ok(Nest::Bytes(&encoding1[..]))
            )
        });
        let encoding2 = [1; 65792];
        create_fixed_template_then(TemplateId::FixedBeta, 65792, |template| {
            assert_eq!(
                template.decode(&encoding2),
                Ok(Nest::Bytes(&encoding2[..])))
        });
    }

    #[test]
    fn should_error_when_encode_incorrect_bytes_alpha() {
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[])),
                Err(Error::fixed__encode_to__bytes_length_should_match_self_length)
            )
        });
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1, 1])),
                Err(Error::fixed__encode_to__bytes_length_should_match_self_length)
            )
        });
    }

    #[test]
    fn should_error_when_encode_incorrect_bytes_beta() {
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[])),
                Err(Error::fixed__encode_to__bytes_length_should_match_self_length)
            )
        });
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1, 1])),
                Err(Error::fixed__encode_to__bytes_length_should_match_self_length))
        });
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1; 258])),
                Err(Error::fixed__encode_to__bytes_length_should_match_self_length))
        });
    }

    #[test]
    fn should_error_when_decode_not_enough_bytes() {
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(template.decode(&[]), Err(Error::fixed__decode_with_remainder__bytes_length_should_be_gte_self_length))
        });
    }

    #[test]
    fn should_jinyang_alpha() {
        let template1 = Template::from_jinyang(&[0, 0]).unwrap();
        let fixed1 : &Fixed = template1.encoder().as_any().downcast_ref().unwrap();
        assert_eq!(template1.id(), 0);
        assert_eq!(fixed1.length(), 1);
        assert_eq!(template1.export_jinyang(), vec![0, 0]);

        let template256 = Template::from_jinyang(&[0, 255]).unwrap();
        let fixed256 : &Fixed = template256.encoder().as_any().downcast_ref().unwrap();
        assert_eq!(template256.id(), 0);
        assert_eq!(fixed256.length(), 256);
        assert_eq!(template256.export_jinyang(), vec![0, 255]);
    }

    #[test]
    fn should_jinyang_beta() {
        let template257 = Template::from_jinyang(&[1, 0, 0]).unwrap();
        let fixed257 : &Fixed = template257.encoder().as_any().downcast_ref().unwrap();
        assert_eq!(template257.id(), 1);
        assert_eq!(fixed257.length(), 257);
        assert_eq!(template257.export_jinyang(), vec![1, 0, 0]);

        let template65792 = Template::from_jinyang(&[1, 255, 255]).unwrap();
        let fixed65792 : &Fixed = template65792.encoder().as_any().downcast_ref().unwrap();
        assert_eq!(template65792.id(), 1);
        assert_eq!(fixed65792.length(), 65792);
        assert_eq!(template65792.export_jinyang(), vec![1, 255, 255]);
    }



}
