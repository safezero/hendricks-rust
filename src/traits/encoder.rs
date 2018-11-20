use error::Error;
use std::any::Any;
use nest::Nest;

pub trait Encoder : Any {
    fn template_id(&self) -> u8;
    fn encode_to<'a>(&self, nest: &Nest, &'a mut Vec<u8>) -> Result<(), Error>;
    fn decode_with_remainder<'a>(&self, bytes: &'a[u8]) -> Result<(Nest<'a>, &'a[u8]), Error>;
    fn export_jinyang_to<'a>(&self, to: &'a mut Vec<u8>);
    fn as_any(&self) -> &dyn Any;
}
