use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: knowit <luke_nr>")]
    CliUsage,
    #[error("No valid solution found")]
    SolverError,
    #[error("This luke is not implemented yet")]
    ImplementationError,
}