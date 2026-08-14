use crate::error::EnactError;
use std::str::FromStr;

#[derive(Debug)]
pub enum TypeReader {
    Texture2D,
    Effect,
}

impl FromStr for TypeReader {
    type Err = EnactError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Microsoft.Xna.Framework.Content.Texture2DReader" => Self::Texture2D,
            "Microsoft.Xna.Framework.Content.EffectReader" => Self::Effect,

            other => return Err(EnactError::UnknownTypeReader(other.to_owned())),
        })
    }
}
