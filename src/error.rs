#[derive(Debug, PartialEq)]
pub enum Error {
    template__decode__should_not_have_any_remainder,
    fixed__new__invalid_template_id,
    fixed__encode_to__bytes_length_should_match_self_length,
    fixed__decode_with_remainder__bytes_length_should_be_gte_self_length,
    fixed__new__alpha__length_too_small,
    fixed__new__alpha__length_too_big,
    fixed__new__beta__length_too_small,
    fixed__new__beta__length_too_big,
    dynamic__encode_to__length_encoding_should_be_less_than_length_encoding_length,
    dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length,
    dynamic__decode_with_remainder__length_encoding_larger_than_remaining_bytes
}
