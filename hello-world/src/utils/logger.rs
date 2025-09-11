use std::error::Error;
use env_logger::*;

pub fn init_logger() -> Result<(), Box<dyn Error>>{
    Builder::from_default_env().init();
    Ok(())
}