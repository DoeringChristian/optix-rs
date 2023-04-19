use std::fmt::Display;

use thiserror::Error;

use crate::OptixResult;

#[derive(Debug, Error)]
pub struct OptixError(OptixResult);

impl Display for OptixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Optix error: {:?}({})", self.0, self.0 as u32)
    }
}

impl OptixResult {
    pub fn check(self) -> Result<(), OptixError> {
        match self {
            OptixResult::OPTIX_SUCCESS => Ok(()),
            _ => Err(OptixError(self)),
        }
    }
}
