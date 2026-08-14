use crate::error::EnactError;
use std::str::FromStr;

#[derive(Debug)]
pub enum TypeReaders {
    Texture2D,
}

impl FromStr for TypeReaders {
    type Err = EnactError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Microsoft.Xna.Framework.Content.Texture2DReader" => Self::Texture2D,

            other => return Err(EnactError::UnknownTypeReader(other.to_owned())),
        })
    }
}
