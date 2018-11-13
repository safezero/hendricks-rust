use error::Error;
use traits::encoder::Encoder;
use encoders::fixed::Fixed;
use num_traits::FromPrimitive;
use template_ids::TemplateId;

pub struct Template<'a> {
    encoder: &'a Encoder
}

impl<'a> Template<'a> {
    pub fn new(encoder: &'a Encoder) -> Template{
        Template {
            encoder
        }
    }
    pub fn from_jinyang(jinyang: &'a [u8]) -> Result<(Template), Error> {
        Ok(Self::from_jinyang_with_remainder(jinyang).unwrap().0)
    }
    pub fn from_jinyang_with_remainder(jinyang: &'a [u8]) -> Result<(Template, &'a [u8]), Error> {
        let template_id = TemplateId::from_u8(jinyang[0]).unwrap();
        let mut encoder_and_remainder;
        match(template_id) {
            TemplateId::FixedAlpha | TemplateId::FixedBeta => {
                encoder_and_remainder = Fixed::from_jinyang_with_remainder(
                    template_id,
                    &jinyang[1..]
                ).unwrap();
            },
            _ => {
                panic!();
            }
        }
        let encoder = encoder_and_remainder.0;
        Ok((
            Template::new(&encoder),
            encoder_and_remainder.1
        ))
    }
    pub fn id(&self) -> u8 {
        self.encoder.template_id()
    }
    pub fn encoder(&self) -> &'a Encoder {
        self.encoder
    }
    pub fn encode(&self, bytes:&[u8]) -> Result<Vec<u8>, Error> {
        let mut encoding = vec![];
        let result = self.encoder.encode_to(&bytes, &mut encoding);
        match result {
            Err(error) => Err(error),
            Ok(_) => Ok(encoding)
        }
    }
    pub fn decode(&self, bytes: &'a [u8]) -> Result<&'a [u8], Error> {
        match self.encoder.decode_with_remainder(&bytes) {
            Err(error) => Err(error),
            Ok(tuple) => {
                if(tuple.1.len() > 0) {
                    Err(Error::template__decode__should_not_have_any_remainder)
                } else {
                    Ok(tuple.0)
                }
            }
        }
    }
    pub fn export_jinyang(&self) -> Vec<u8> {
        let mut jinyang = vec![];
        self.export_jinyang_to(&mut jinyang);
        jinyang
    }
    pub fn encode_to<'b>(&self, bytes: &[u8], to: &'b mut Vec<u8>) -> Result<(), Error> {
        to.push(self.encoder.template_id());
        self.encoder.encode_to(bytes, to)
    }
    pub fn decode_with_remainder<'b>(&self, bytes: &'b [u8]) -> Result<(&'b [u8], &'b [u8]), Error> {
        self.encoder.decode_with_remainder(bytes)
    }
    pub fn export_jinyang_to<'b>(&self, to: &'b mut Vec<u8>) {
        to.push(self.encoder.template_id());
        self.encoder.export_jinyang_to(to);
    }
}
