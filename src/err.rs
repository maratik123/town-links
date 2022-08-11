use log::SetLoggerError;
use wgpu::{RequestDeviceError, SurfaceError};
use winit::error::{ExternalError, OsError};

#[derive(Debug)]
pub enum Error {
    LogSetLoggerError(SetLoggerError),
    RequestAdapterError,
    WgpuRequestDeviceError(RequestDeviceError),
    WgpuSurfaceError(SurfaceError),
    WinitOsError(OsError),
    WinitExternalError(ExternalError),
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

impl From<SurfaceError> for Error {
    #[inline]
    fn from(err: SurfaceError) -> Self {
        Self::WgpuSurfaceError(err)
    }
}

impl From<ExternalError> for Error {
    #[inline]
    fn from(err: ExternalError) -> Self {
        Self::WinitExternalError(err)
    }
}
