use crate::core::*;

pub struct ColorTargetMultisample<C: TextureDataType>(RenderTargetMultisample<C, f32>);

impl<C: TextureDataType + Default> ColorTargetMultisample<C> {
    pub fn new(context: &Context, width: u32, height: u32, number_of_samples: u32) -> Self {
        Self(RenderTargetMultisample::new_color(
            context,
            width,
            height,
            number_of_samples,
        ))
    }

    /// The width of this target.
    pub fn width(&self) -> u32 {
        self.0.width()
    }

    /// The height of this target.
    pub fn height(&self) -> u32 {
        self.0.height()
    }

    /// The number of samples for each fragment.
    pub fn number_of_samples(&self) -> u32 {
        self.0.number_of_samples()
    }

    pub fn as_render_target(&mut self) -> RenderTarget<'_> {
        self.0.as_render_target()
    }

    pub fn resolve(&self) -> Texture2D {
        self.0.resolve_color()
    }
}
