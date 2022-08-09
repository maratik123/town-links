use glium::{
    backend::glutin::DisplayCreationError, glutin::error::ExternalError, SwapBuffersError,
};
use imgui_glium_renderer::RendererError;

#[derive(Debug)]
pub enum Error {
    GliumDisplayCreationError(DisplayCreationError),
    GliumRendererError(RendererError),
    GliumExternalError(ExternalError),
    GliumSwapBuffersError(SwapBuffersError),
}

impl From<DisplayCreationError> for Error {
    #[inline]
    fn from(err: DisplayCreationError) -> Self {
        Error::GliumDisplayCreationError(err)
    }
}

impl From<RendererError> for Error {
    #[inline]
    fn from(err: RendererError) -> Self {
        Error::GliumRendererError(err)
    }
}

impl From<ExternalError> for Error {
    #[inline]
    fn from(err: ExternalError) -> Self {
        Error::GliumExternalError(err)
    }
}

impl From<SwapBuffersError> for Error {
    #[inline]
    fn from(err: SwapBuffersError) -> Self {
        Error::GliumSwapBuffersError(err)
    }
}
