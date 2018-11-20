#[derive(Debug)]
pub enum Nest<'a> {
    Bytes(&'a [u8]),
    Nests(&'a [&'a Nest<'a>])
}

impl<'a> Nest<'a> {
    pub fn bytes(&self) -> &[u8]  {
        match (&self) {
            Nest::Bytes(bytes) => &bytes,
            _ => panic!()
        }
    }
    pub fn nests(&self) -> &[&Nest]  {
        match (&self) {
            Nest::Nests(nests) => nests,
            _ => panic!()
        }
    }
}

impl<'a, 'b> PartialEq<Nest<'a>> for Nest<'b> {
    fn eq(&self, other: &Nest) -> bool {
        match (&self, other) {
            (&Nest::Bytes(self_bytes), &Nest::Bytes(other_bytes)) => {
                self_bytes.iter().zip(other_bytes.iter()).all(|(a,b)| a == b)
            },
            (&Nest::Nests(self_nests), &Nest::Nests(other_nests)) => {
                self_nests.iter().zip(other_nests.iter()).all(|(a,b)| a == b)
            },
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Nest;

    #[test]
    fn equal_bytes_should_be_equal() {
        assert_eq!(
            Nest::Bytes(&[1, 2, 3, 4]),
            Nest::Bytes(&[1, 2, 3, 4])
        );
    }

    #[test]
    fn unequal_bytes_should_not_be_equal() {
        assert_ne!(
            Nest::Bytes(&[1, 2, 3, 4]),
            Nest::Bytes(&[4, 3, 2, 1])
        );
    }

    #[test]
    fn equal_nests_should_be_equal() {
        assert_eq!(
            Nest::Nests(&[
                &Nest::Bytes(&[1, 2, 3, 4]),
                &Nest::Nests(&[
                    &Nest::Bytes(&[5, 6]),
                    &Nest::Bytes(&[7, 8])
                ])
            ]),
            Nest::Nests(&[
                &Nest::Bytes(&[1, 2, 3, 4]),
                &Nest::Nests(&[
                    &Nest::Bytes(&[5, 6]),
                    &Nest::Bytes(&[7, 8])
                ])
            ]),
        );
    }

    #[test]
    fn unequal_nests_should_not_be_equal() {
        assert_ne!(
            Nest::Nests(&[
                &Nest::Bytes(&[1, 2, 3, 4]),
                &Nest::Nests(&[
                    &Nest::Bytes(&[5, 6]),
                    &Nest::Bytes(&[7, 8])
                ])
            ]),
            Nest::Nests(&[
                &Nest::Bytes(&[1, 2, 3, 4]),
                &Nest::Nests(&[
                    &Nest::Bytes(&[5, 6]),
                    &Nest::Bytes(&[7, 9])
                ])
            ]),
        );
    }

}
