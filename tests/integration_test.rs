extern crate hendricks;

mod fixed {

    use hendricks::template_ids::TemplateId;
    use hendricks::template::Template;
    use hendricks::encoders::fixed::Fixed;
    use hendricks::traits::encoder::Encoder;
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

mod dynamic {

    use hendricks::template_ids::TemplateId;
    use hendricks::template::Template;
    use hendricks::encoders::dynamic::Dynamic;
    use hendricks::traits::encoder::Encoder;
    use hendricks::error::Error;

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
            assert_eq!(template.encode(&[]), Ok(vec![0]));
            assert_eq!(template.decode(&[0][..]), Ok(&[][..]))
        });
        create_template_then(TemplateId::DynamicAlpha, |template| {
            let mut vec1= vec![255];
            let mut vec2 = vec![1; 255];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 255]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 255][..]));
        });
    }

    #[test]
    fn should_encode_beta() {
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(template.encode(&[]), Ok(vec![0, 0]));
            assert_eq!(template.decode(&[0, 0][..]), Ok(&[][..]))
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            let mut vec1= vec![255, 0];
            let mut vec2 = vec![1; 255];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 255]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 255][..]));
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            let mut vec1= vec![0, 1];
            let mut vec2 = vec![1; 256];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 256]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 256][..]));

        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            let mut vec1= vec![255, 255];
            let mut vec2 = vec![1; 65535];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 65535]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 65535][..]));
        });
    }

    #[test]
    fn should_encode_gamma() {
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(template.encode(&[]), Ok(vec![0, 0, 0]));
            assert_eq!(template.decode(&[0, 0, 0][..]), Ok(&[][..]))
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![255, 0, 0];
            let mut vec2 = vec![1; 255];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 255]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 255][..]));
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![0, 1, 0];
            let mut vec2 = vec![1; 256];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 256]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 256][..]));
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![255, 255, 0];
            let mut vec2 = vec![1; 65535];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 65535]), Ok(vec1.clone()));
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![0, 0, 1];
            let mut vec2 = vec![1; 65536];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 65536]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 65536][..]));
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            let mut vec1= vec![255, 255, 255];
            let mut vec2 = vec![1; 16777215];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 16777215]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 16777215][..]));
        });
    }

    #[test]
    fn should_encode_delta() {
        create_template_then(TemplateId::DynamicDelta, |template| {
            assert_eq!(template.encode(&[]), Ok(vec![0, 0, 0, 0]));
            assert_eq!(template.decode(&[0, 0, 0, 0][..]), Ok(&[][..]))
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![255, 0, 0, 0];
            let mut vec2 = vec![1; 255];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 255]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 255][..]));
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![0, 1, 0, 0];
            let mut vec2 = vec![1; 256];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 256]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 256][..]));
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![255, 255, 0, 0];
            let mut vec2 = vec![1; 65535];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 65535]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 65535][..]));
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![0, 0, 1, 0];
            let mut vec2 = vec![1; 65536];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 65536]), Ok(vec1.clone()));
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![255, 255, 255, 0];
            let mut vec2 = vec![1; 16777215];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 16777215]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 16777215][..]));
        });
        create_template_then(TemplateId::DynamicDelta, |template| {
            let mut vec1= vec![0, 0, 0, 1];
            let mut vec2 = vec![1; 16777216];
            vec1.append(&mut vec2);
            assert_eq!(template.encode(&vec![1; 16777216]), Ok(vec1.clone()));
            assert_eq!(template.decode(&vec1[..]), Ok(&[1; 16777216][..]));
        });
        // create_template_then(TemplateId::DynamicDelta, |template| {
        //     let mut vec1= vec![255, 255, 255, 255];
        //     let mut vec2 = vec![1; 4294967295];
        //     vec1.append(&mut vec2);
        //     assert_eq!(template.encode(&vec![1; 4294967295]), Ok(vec1.clone()));
        //     assert_eq!(template.decode(&vec1[..]), Ok(&[1; 4294967295][..]));
        // });
    }

    #[test]
    fn should_error_when_encode_too_many_bytes_alpha() {
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(template.encode(&[1; 256]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(template.encode(&[1; 65792]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(template.encode(&[1; 16777216]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        });
        // create_template_then(TemplateId::DynamicDelta, |template| {
        // TODO: Memory Error
        //     assert_eq!(template.encode(&[1; 4294967295]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        // });
    }

    #[test]
    fn should_error__encode_to__bytes_length_should_be_lte_max_length() {
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(template.encode(&[1; 256]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        });
        create_template_then(TemplateId::DynamicBeta, |template| {
            assert_eq!(template.encode(&[1; 65792]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        });
        create_template_then(TemplateId::DynamicGamma, |template| {
            assert_eq!(template.encode(&[1; 16777216]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        });
        // create_template_then(TemplateId::DynamicDelta, |template| {
        // TODO: Memory Error
        //     assert_eq!(template.encode(&[1; 4294967295]), Err(Error::dynamic__encode_to__bytes_length_should_be_lte_max_length))
        // });
    }

    #[test]
    fn should_error__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length() {
        create_template_then(TemplateId::DynamicAlpha, |template| {
            assert_eq!(template.decode(&[]), Err(Error::dynamic__decode_with_remainder__bytes_length_should_be_gte_length_encoding_length))
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
