use std::convert::TryFrom;
use std::num::ParseIntError;
use std::str::FromStr;

use crc::{Crc, CRC_16_GSM};

pub struct Element(u32);

impl Element {
    pub fn new(id: u16) -> Self {
        let digest = {
            let crc = Crc::<u16>::new(&CRC_16_GSM);
            let mut digest = crc.digest();
            digest.update(&id.to_ne_bytes());
            digest.finalize()
        };

        Element((u32::from(id) << 16) | digest as u32)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for Element {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let id = value >> 16;

        let digest = {
            let crc: Crc<u16> = Crc::<u16>::new(&CRC_16_GSM);
            let mut digest = crc.digest();
            digest.update(&id.to_ne_bytes());
            digest.finalize()
        };

        if value as u16 == digest {
            Ok(Element(value))
        } else {
            Err(())
        }
    }
}

impl FromStr for Element {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: u16 = s.parse()?;

        Ok(Self::new(id))
    }
}
