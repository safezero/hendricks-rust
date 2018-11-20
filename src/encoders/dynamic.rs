use error::Error;
use traits::encoder::Encoder;
use template_ids::TemplateId;
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use std::any::Any;
use nest::Nest;

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
    pub fn encode_length_to(&self, length: usize, to: &mut Vec<u8>) {
        if (length > std::u32::MAX as usize) {
            panic!();
        }
        let mut length_encoding = Vec::new();
        length_encoding.write_u32::<LittleEndian>(length as u32);
        to.extend_from_slice(&length_encoding[0..self.length_encoding_length]);
    }
}

impl Encoder for Dynamic {
    fn template_id(&self) -> u8 {
        self.template_id as u8
    }
    fn encode_to<'a>(&self, nest: &Nest, to: &'a mut Vec<u8>) -> Result<(), Error> {
        let bytes = nest.bytes();
        if bytes.len() > self.max_length {
            Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length)
        } else {
            self.encode_length_to(bytes.len(), to);
            to.extend_from_slice(&bytes);
            Ok(())
        }
    }
    fn decode_with_remainder<'a>(&self, bytes: &'a [u8]) -> Result<(Nest<'a>, &'a [u8]), Error> {
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
                    Nest::Bytes(&bytes[self.length_encoding_length..self.length_encoding_length + length]),
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

#[cfg(test)]
mod tests {

    use template_ids::TemplateId;
    use template::Template;
    use encoders::dynamic::Dynamic;
    use traits::encoder::Encoder;
    use error::Error;
    use nest::Nest;

    fn create_template_then<F>(
        template_id: TemplateId,
        then: F
    ) where F: Fn(Template) {
        match template_id {
            TemplateId::DynamicAlpha
            | TemplateId::DynamicBeta
            | TemplateId::DynamicGamma
            | TemplateId::DynamicDelta => {
                let encoderResult = Dynamic::new(template_id);
                match encoderResult {
                    Ok(encoder) => {
                        then(Template::new(Box::new(encoder)));
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
            Dynamic::new(TemplateId::FixedAlpha).err(),
            Some(Error::dynamic__new__invalid_template_id)
        );
    }

    #[test]
    fn should_create_dynamics() {
        assert_eq!(
            Dynamic::new(TemplateId::DynamicAlpha).err(),
            None
        );
        assert_eq!(
            Dynamic::new(TemplateId::DynamicBeta).err(),
            None
        );
        assert_eq!(
            Dynamic::new(TemplateId::DynamicGamma).err(),
            None
        );
        assert_eq!(
            Dynamic::new(TemplateId::DynamicDelta).err(),
            None
        );
    }


    #[test]
    fn should_encode_decode_alpha() {
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[])),
                Ok(vec![0])
            );
            assert_eq!(
                template.decode(&[0]),
                Ok(Nest::Bytes(&[]))
            )
        });
        create_template_then(TemplateId::DynamicAlpha, |template| {
            let mut vec1= vec![255];
            let mut vec2 = vec![1; 255];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(
                &Nest::Bytes(&vec![1; 255])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 255][..]))
            );
        });
    }

    #[test]
    fn should_encode_beta() {
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[])),
                Ok(vec![0, 0])
            );
            assert_eq!(template.decode(&[0, 0][..]), Ok(Nest::Bytes(&[][..])))
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            let mut vec1= vec![255, 0];
            let mut vec2 = vec![1; 255];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 255])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 255][..]))
            );
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            let mut vec1= vec![0, 1];
            let mut vec2 = vec![1; 256];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 256])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 256][..]))
            );
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            let mut vec1= vec![255, 255];
            let mut vec2 = vec![1; 65535];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 65535])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 65535][..])));
        });
    }

    #[test]
    fn should_encode_gamma() {
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[])),
                Ok(vec![0, 0, 0])
            );
            assert_eq!(template.decode(&[0, 0, 0][..]), Ok(Nest::Bytes(&[][..])))
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![255, 0, 0];
            let mut vec2 = vec![1; 255];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 255])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 255][..]))
            );
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![0, 1, 0];
            let mut vec2 = vec![1; 256];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 256])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 256][..]))
            );
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![255, 255, 0];
            let mut vec2 = vec![1; 65535];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 65535])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 65535][..])));
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![0, 0, 1];
            let mut vec2 = vec![1; 65536];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 65536])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 65536][..])));
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![255, 255, 255];
            let mut vec2 = vec![1; 16777215];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 16777215])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 16777215][..])));
        });
    }

    #[test]
    fn should_encode_delta() {
        create_template_then(TemplateId::DynamicDelta, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[])),
                Ok(vec![0, 0, 0, 0])
            );
            assert_eq!(template.decode(&[0, 0, 0, 0][..]), Ok(Nest::Bytes(&[][..])))
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![255, 0, 0, 0];
            let mut vec2 = vec![1; 255];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 255])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 255][..]))
            );
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![0, 1, 0, 0];
            let mut vec2 = vec![1; 256];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 256])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 256][..]))
            );
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![255, 255, 0, 0];
            let mut vec2 = vec![1; 65535];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 65535])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 65535][..])));
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![0, 0, 1, 0];
            let mut vec2 = vec![1; 65536];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 65536])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 65536][..])));
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![255, 255, 255, 0];
            let mut vec2 = vec![1; 16777215];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 16777215])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 16777215][..])));
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![0, 0, 0, 1];
            let mut vec2 = vec![1; 16777216];
            vec1.append(&mut vec2);
            assert_eq!(
                template.encode(&Nest::Bytes(&vec![1; 16777216])),
                Ok(vec1.clone())
            );
            assert_eq!(
                template.decode(&vec1[..]),
                Ok(Nest::Bytes(&[1; 16777216][..])));
        });
        // TODO: handle huge dynamics (or remove)
        // create_template_then(TemplateId::DynamicDelta, |template| {
        //     let mut vec1= vec![255, 255, 255, 255];
        //     let mut vec2 = vec![1; 4294967295];
        //     vec1.append(&mut vec2);
        //     assert_eq!(
        //         template.encode(&Nest::Bytes(&vec![1; 4294967295])),
        //         Ok(vec1.clone())
        //     );
        //     assert_eq!(
        //         template.decode(&vec1[..]),
        //         Ok(Nest::Bytes(&[1; 4294967295][..])));
        // });
    }

    #[test]
    fn should_error_when_encode_too_many_bytes_alpha() {
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1; 256])),
                Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length)
            )
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1; 65792])),
                Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length)
            )
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1; 16777216])),
                Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length)
            )
        });
        // create_template_then(TemplateId::DynamicDelta, |template| {
        // TODO: Memory Error
        //     assert_eq!(template.encode(&[1; 4294967295]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        // });
    }

    #[test]
    fn should_error__encode_to__bytes_length_should_be_lte_max_length() {
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1; 256])),
                Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length)
            )
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1; 65792])),
                Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length)
            )
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(
                template.encode(&Nest::Bytes(&[1; 16777216])),
                Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length)
            )
        });
        // create_template_then(TemplateId::DynamicDelta, |template| {
        // TODO: Memory Error
        //     assert_eq!(template.encode(&[1; 4294967295]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        // });
    }

    #[test]
    fn should_error__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length() {
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(
                template.decode(&[]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(template.decode(&[]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(template.decode(&[0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(template.decode(&[]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(template.decode(&[0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(template.decode(&[0, 0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            assert_eq!(template.decode(&[]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            assert_eq!(template.decode(&[0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            assert_eq!(template.decode(&[0, 0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            assert_eq!(template.decode(&[0, 0, 0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
        });
    }

    #[test]
    fn should_error__dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length_plus_length() {
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(template.decode(&[1]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length_plus_length))
        });
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(template.decode(&[2, 1]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length_plus_length))
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(template.decode(&[1, 0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length_plus_length))
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(template.decode(&[1, 0, 0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length_plus_length))
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            assert_eq!(template.decode(&[1, 0, 0, 0]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length_plus_length))
        });
    }

    #[test]
    fn should_jinyang() {
        let templateAlpha = Template::from_jinyang(&[2]).unwrap();
        let dynamicAlpha : &Dynamic = templateAlpha.encoder().as_any().downcast_ref().unwrap();
        assert_eq!(templateAlpha.id(), 2);
        assert_eq!(dynamicAlpha.length_encoding_length(), 1);
        assert_eq!(templateAlpha.export_jinyang(), vec![2]);

        let templateBeta = Template::from_jinyang(&[3]).unwrap();
        let dynamicBeta : &Dynamic = templateBeta.encoder().as_any().downcast_ref().unwrap();
        assert_eq!(templateBeta.id(), 3);
        assert_eq!(dynamicBeta.length_encoding_length(), 2);
        assert_eq!(templateBeta.export_jinyang(), vec![3]);

        let templateGamma = Template::from_jinyang(&[4]).unwrap();
        let dynamicGamma : &Dynamic = templateGamma.encoder().as_any().downcast_ref().unwrap();
        assert_eq!(templateGamma.id(), 4);
        assert_eq!(dynamicGamma.length_encoding_length(), 3);
        assert_eq!(templateGamma.export_jinyang(), vec![4]);

        let templateDelta = Template::from_jinyang(&[5]).unwrap();
        let dynamicDelta : &Dynamic = templateDelta.encoder().as_any().downcast_ref().unwrap();
        assert_eq!(templateDelta.id(), 5);
        assert_eq!(dynamicDelta.length_encoding_length(), 4);
        assert_eq!(templateDelta.export_jinyang(), vec![5]);
    }

}
