use asn1_codecs::{PerCodecData, PerCodecError, uper::UperCodec};
use thiserror::Error;
#[allow(warnings, unused, unreachable_patterns, non_camel_case_types)]
pub mod lte_rrc;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Failed to decode UPER data: {0}")]
    UperDecodeError(PerCodecError),
}

pub fn decode<T>(data: &[u8]) -> Result<T, ParsingError>
where
    T: UperCodec<Output = T>,
{
    let mut asn_data = PerCodecData::from_slice_uper(data);
    T::uper_decode(&mut asn_data).map_err(ParsingError::UperDecodeError)
}
