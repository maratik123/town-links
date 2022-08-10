use log::SetLoggerError;
use wgpu::RequestDeviceError;
use winit::error::OsError;

#[derive(Debug)]
pub enum Error {
    LogSetLoggerError(SetLoggerError),
    RequestAdapterError,
    WgpuRequestDeviceError(RequestDeviceError),
    WinitOsError(OsError),
}

impl From<OsError> for Error {
    #[inline]
    fn from(err: OsError) -> Self {
        Self::WinitOsError(err)
    }
}

impl From<RequestDeviceError> for Error {
    #[inline]
    fn from(err: RequestDeviceError) -> Self {
        Self::WgpuRequestDeviceError(err)
    }
}

impl From<SetLoggerError> for Error {
    #[inline]
    fn from(err: SetLoggerError) -> Self {
        Self::LogSetLoggerError(err)
    }
}
