use winit::error::OsError;

#[derive(Debug)]
pub enum Error {
    WinitOsError(OsError),
    CreateAdapter(String),
}

impl From<OsError> for Error {
    #[inline]
    fn from(err: OsError) -> Self {
        Error::WinitOsError(err)
    }
}
