use error::Error;
use traits::encoder::Encoder;
use encoders::fixed::Fixed;
use encoders::dynamic::Dynamic;
use num_traits::FromPrimitive;
use template_ids::TemplateId;
use nest::Nest;

pub struct Template {
    encoder: Box<Encoder>
}

impl Template {
    pub fn new(encoder: Box<Encoder>) -> Template{
        Template {
            encoder
        }
    }
    pub fn from_jinyang<'a>(jinyang: &'a [u8]) -> Result<(Template), Error> {
        Ok(Self::from_jinyang_with_remainder(jinyang).unwrap().0)
    }
    pub fn from_jinyang_with_remainder<'a>(jinyang: &'a [u8]) -> Result<(Template, &'a [u8]), Error> {
        let template_id = TemplateId::from_u8(jinyang[0]).unwrap();
        match(template_id) {
            TemplateId::FixedAlpha | TemplateId::FixedBeta => {
                let encoder_and_remainder = Fixed::from_jinyang_with_remainder(
                    template_id,
                    &jinyang[1..]
                ).unwrap();
                Ok((
                    Template::new(Box::new(encoder_and_remainder.0)),
                    encoder_and_remainder.1
                ))
            },
            TemplateId::DynamicAlpha |
            TemplateId::DynamicBeta |
            TemplateId::DynamicGamma |
            TemplateId::DynamicDelta => {
                let encoder_and_remainder = Dynamic::from_jinyang_with_remainder(
                    template_id,
                    &jinyang[1..]
                ).unwrap();
                Ok((
                    Template::new(Box::new(encoder_and_remainder.0)),
                    encoder_and_remainder.1
                ))
            },
            _ => {
                panic!();
            }
        }
    }
    pub fn id(&self) -> u8 {
        self.encoder.template_id()
    }
    pub fn encoder(&self) -> &Box<Encoder> {
        &self.encoder
    }
    pub fn encode(&self, nest:&Nest) -> Result<Vec<u8>, Error> {
        let mut encoding = vec![];
        let result = self.encoder.encode_to(&nest, &mut encoding);
        match result {
            Err(error) => Err(error),
            Ok(_) => Ok(encoding)
        }
    }
    pub fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Nest<'a>, Error> {
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
    pub fn encode_to<'a>(&self, nest: &Nest, to: &'a mut Vec<u8>) -> Result<(), Error> {
        to.push(self.encoder.template_id());
        self.encoder.encode_to(nest, to)
    }
    pub fn decode_with_remainder<'a>(&self, bytes: &'a [u8]) -> Result<(Nest<'a>, &'a [u8]), Error> {
        self.encoder.decode_with_remainder(bytes)
    }
    pub fn export_jinyang_to<'a>(&self, to: &'a mut Vec<u8>) {
        to.push(self.encoder.template_id());
        self.encoder.export_jinyang_to(to);
    }
}
