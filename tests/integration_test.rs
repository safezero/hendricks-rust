extern crate hendricks;

mod tests {

    use hendricks::template_ids::TemplateId;
    use hendricks::template::Template;
    use hendricks::encoders::fixed::Fixed;
    use hendricks::error::Error;

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
            assert_eq!(template.encode(&[1]), Ok(vec![1]))
        });
        create_fixed_template_then(TemplateId::FixedAlpha, 256, |template| {
            assert_eq!(template.encode(&[1; 256]), Ok(vec![1; 256]))
        });
    }

    #[test]
    fn should_encode_beta() {
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(template.encode(&[1; 257]), Ok(vec![1; 257]))
        });
        create_fixed_template_then(TemplateId::FixedBeta, 65792, |template| {
            assert_eq!(template.encode(&[1; 65792]), Ok(vec![1; 65792]))
        });
    }

    #[test]
    fn should_decode_alpha() {
        let encoding1 = [1];
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(template.decode(&encoding1), Ok(&encoding1[..]))
        });
        let encoding2 = [1; 256];
        create_fixed_template_then(TemplateId::FixedAlpha, 256, |template| {
            assert_eq!(template.decode(&encoding2), Ok(&encoding2[..]))
        });
    }

    #[test]
    fn should_decode_beta() {
        let encoding1 = [1; 257];
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(template.decode(&encoding1), Ok(&encoding1[..]))
        });
        let encoding2 = [1; 65792];
        create_fixed_template_then(TemplateId::FixedBeta, 65792, |template| {
            assert_eq!(template.decode(&encoding2), Ok(&encoding2[..]))
        });
    }

    #[test]
    fn should_error_when_encode_incorrect_bytes_alpha() {
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(template.encode(&[]), Err(Error::fixed__encode_to__bytes_length_should_match_self_length))
        });
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(template.encode(&[1, 1]), Err(Error::fixed__encode_to__bytes_length_should_match_self_length))
        });
    }

    #[test]
    fn should_error_when_encode_incorrect_bytes_beta() {
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(template.encode(&[]), Err(Error::fixed__encode_to__bytes_length_should_match_self_length))
        });
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(template.encode(&[1, 1]), Err(Error::fixed__encode_to__bytes_length_should_match_self_length))
        });
        create_fixed_template_then(TemplateId::FixedBeta, 257, |template| {
            assert_eq!(template.encode(&[1; 258]), Err(Error::fixed__encode_to__bytes_length_should_match_self_length))
        });
    }

    #[test]
    fn should_error_when_decode_not_enough_bytes() {
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(template.decode(&[]), Err(Error::fixed__decode_with_remainder__bytes_length_should_be_gte_self_length))
        });
    }

    #[test]
    fn should_export_jinyang_alpha() {
        create_fixed_template_then(TemplateId::FixedAlpha, 1, |template| {
            assert_eq!(template.export_jinyang(), vec![0, 0]);
        });
        create_fixed_template_then(TemplateId::FixedAlpha, 256, |template| {
            assert_eq!(template.export_jinyang(), vec![0, 255]);
        });
    }

    #[test]
    fn should_from_jinyang_alpha() {
        // let template1 = Template::from_jinyang(&[0, 0]).unwrap();
        // let encoder1: &Fixed = template1.encoder();
        // assert_eq!(template1.id(), 0);
        // assert_eq!(encoder1.length(), 1);
    }

}
