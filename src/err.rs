use winit::error::OsError;

#[derive(Debug)]
pub enum Error {
    WinitOsError(OsError),
}

impl From<OsError> for Error {
    #[inline]
    fn from(err: OsError) -> Self {
        Error::WinitOsError(err)
    }
}
