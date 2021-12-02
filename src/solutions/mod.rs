mod luke1;
mod utils;
use crate::Error;


pub fn solver(luke: u8) -> Result<String, Error> {
    match luke {
        1 => {
                let mut luke = luke1::Luke1::new();
                luke.solve()?;
                Ok(luke.answer)
            },
        _ => Err(Error::ImplementationError)
    }
}   