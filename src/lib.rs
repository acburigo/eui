#[derive(Debug, PartialEq)]
pub struct EUI48([u8; 6]);

#[derive(Debug, PartialEq)]
pub struct EUI64([u8; 8]);

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidHexCharacter,
    InvalidStringLength,
    OddLength,
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        match e {
            hex::FromHexError::InvalidHexCharacter { .. } => {
                Error::InvalidHexCharacter
            }
            hex::FromHexError::InvalidStringLength => Error::InvalidStringLength,
            hex::FromHexError::OddLength => Error::OddLength,
        }
    }
}

pub trait EUI {
    fn to_bytes(&self) -> &[u8];

    fn to_canonical_fmt(&self) -> String {
        self.to_bytes().iter().fold(String::new(), |acc, new| {
            if acc.is_empty() {
                format!("{:02X}", new)
            } else {
                format!("{}-{:02X}", acc, new)
            }
        })
    }

    fn to_colon_fmt(&self) -> String {
        self.to_bytes().iter().fold(String::new(), |acc, new| {
            if acc.is_empty() {
                format!("{:02X}", new)
            } else {
                format!("{}:{:02X}", acc, new)
            }
        })
    }

    fn to_dot_fmt(&self) -> String {
        self.to_bytes().chunks(2).fold(String::new(), |acc, new| {
            if acc.is_empty() {
                format!("{:02X}{:02X}", new[0], new[1])
            } else {
                format!("{}.{:02X}{:02X}", acc, new[0], new[1])
            }
        })
    }
}

impl EUI for EUI48 {
    fn to_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl EUI for EUI64 {
    fn to_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl std::convert::TryFrom<&str> for EUI48 {
    type Error = crate::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.replace(&['.', ':', '-'][..], "");
        let bytes = hex::decode(s).map_err(Error::from)?;

        if bytes.len() != 6 {
            return Err(Self::Error::InvalidStringLength);
        }

        let mut raw_address: [u8; 6] = Default::default();
        raw_address.copy_from_slice(bytes.as_slice());
        Ok(EUI48(raw_address))
    }
}

impl std::convert::TryFrom<&str> for EUI64 {
    type Error = crate::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.replace(&['.', ':', '-'][..], "");
        let bytes = hex::decode(s).map_err(Error::from)?;

        if bytes.len() != 8 {
            return Err(Self::Error::InvalidStringLength);
        }

        let mut raw_address: [u8; 8] = Default::default();
        raw_address.copy_from_slice(bytes.as_slice());
        Ok(EUI64(raw_address))
    }
}

#[cfg(test)]
mod tests {
    use crate::{EUI, EUI48, EUI64, Error};
    use std::convert::TryFrom;

    #[test]
    fn eui48_to_canonical_fmt() {
        let eui = EUI48([0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F]);
        assert_eq!(eui.to_canonical_fmt(), "0A-1B-2C-3D-4E-5F");
    }

    #[test]
    fn eui48_to_colon_fmt() {
        let eui = EUI48([0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F]);
        assert_eq!(eui.to_colon_fmt(), "0A:1B:2C:3D:4E:5F");
    }

    #[test]
    fn eui48_to_dot_fmt() {
        let eui = EUI48([0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F]);
        assert_eq!(eui.to_dot_fmt(), "0A1B.2C3D.4E5F");
    }

    #[test]
    fn eui64_to_canonical_fmt() {
        let eui = EUI64([0x00, 0xFF, 0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F]);
        assert_eq!(eui.to_canonical_fmt(), "00-FF-0A-1B-2C-3D-4E-5F");
    }

    #[test]
    fn eui64_to_colon_fmt() {
        let eui = EUI64([0x00, 0xFF, 0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F]);
        assert_eq!(eui.to_colon_fmt(), "00:FF:0A:1B:2C:3D:4E:5F");
    }

    #[test]
    fn eui64_to_dot_fmt() {
        let eui = EUI64([0x00, 0xFF, 0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F]);
        assert_eq!(eui.to_dot_fmt(), "00FF.0A1B.2C3D.4E5F");
    }

    #[test]
    fn eui48_from_canonical_fmt() {
        assert_eq!(
            EUI48::try_from("0A-1B-2C-3D-4E-5F").unwrap(),
            EUI48([0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F])
        );
    }

    #[test]
    fn eui48_from_colon_fmt() {
        assert_eq!(
            EUI48::try_from("0A:1B:2C:3D:4E:5F").unwrap(),
            EUI48([0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F])
        );
    }

    #[test]
    fn eui48_from_dot_fmt() {
        assert_eq!(
            EUI48::try_from("0A1B.2C3D.4E5F").unwrap(),
            EUI48([0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F])
        );
    }

    #[test]
    fn eui64_from_canonical_fmt() {
        assert_eq!(
            EUI64::try_from("00-FF-0A-1B-2C-3D-4E-5F").unwrap(),
            EUI64([0x00, 0xFF, 0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F])
        );
    }

    #[test]
    fn eui64_from_colon_fmt() {
        assert_eq!(
            EUI64::try_from("00:FF:0A:1B:2C:3D:4E:5F").unwrap(),
            EUI64([0x00, 0xFF, 0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F])
        );
    }

    #[test]
    fn eui64_from_dot_fmt() {
        assert_eq!(
            EUI64::try_from("00FF.0A1B.2C3D.4E5F").unwrap(),
            EUI64([0x00, 0xFF, 0x0A, 0x1B, 0x2C, 0x3D, 0x4E, 0x5F])
        );
    }

    #[test]
    fn eui48_from_canonical_fmt_bad_character() {
        assert_eq!(
            EUI48::try_from("0A-1B-2C-3D-4x-5F"),
            Err(Error::InvalidHexCharacter)
        );
    }

    #[test]
    fn eui48_from_canonical_fmt_missing_byte() {
        assert_eq!(
            EUI48::try_from("0A-1B-2C-3D-4E"),
            Err(Error::InvalidStringLength)
        );
    }

    #[test]
    fn eui48_from_canonical_fmt_missing_character() {
        assert_eq!(
            EUI48::try_from("0A-1B-2C-3D-4E-5"),
            Err(Error::OddLength)
        );
    }

    #[test]
    fn eui64_from_canonical_fmt_bad_character() {
        assert_eq!(
            EUI64::try_from("00-FF-0A-1B-2C-3D-4x-5F"),
            Err(Error::InvalidHexCharacter)
        );
    }

    #[test]
    fn eui64_from_canonical_fmt_missing_byte() {
        assert_eq!(
            EUI64::try_from("00-FF-0A-1B-2C-3D-4E"),
            Err(Error::InvalidStringLength)
        );
    }

    #[test]
    fn eui64_from_canonical_fmt_missing_character() {
        assert_eq!(
            EUI64::try_from("00-FF-0A-1B-2C-3D-4E-5"),
            Err(Error::OddLength)
        );
    }
}
