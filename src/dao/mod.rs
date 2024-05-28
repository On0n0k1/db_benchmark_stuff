pub mod paciente;

use crate::error::Error;
use crate::globals;
use tiberius::Config;

pub struct Dao {
    config: Config,
}

impl Dao {
    pub fn new() -> Result<Self, Error> {
        let config = Config::from_ado_string(globals::db::ADO).or_else(Error::ado)?;
        Ok(Self { config })
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
}
