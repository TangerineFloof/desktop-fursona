use glutin::context::NotCurrentContext;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use std::ops::Deref;

pub struct GlContextWrapper {
    not_current_gl_context: Option<NotCurrentContext>,
}

impl GlContextWrapper {
    pub fn new(
        gl_config: &glutin::config::Config,
        gl_display: &glutin::display::Display,
        raw_window_handle: Option<raw_window_handle::RawWindowHandle>,
    ) -> Self {
        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(raw_window_handle);

        let not_current_gl_context = Some(unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .unwrap_or_else(|_| {
                            gl_display
                                .create_context(&gl_config, &legacy_context_attributes)
                                .expect("failed to create context")
                        })
                })
        });

        Self {
            not_current_gl_context,
        }
    }

    pub fn make_current(&mut self, gl_surface: &Surface<WindowSurface>) -> CurrentGlContext {
        CurrentGlContext {
            current: Some(
                self.not_current_gl_context
                    .take()
                    .unwrap()
                    .make_current(gl_surface)
                    .unwrap(),
            ),
            not_current_gl_context: &mut self.not_current_gl_context,
        }
    }
}

pub struct CurrentGlContext<'a> {
    current: Option<glutin::context::PossiblyCurrentContext>,
    not_current_gl_context: &'a mut Option<NotCurrentContext>,
}

impl<'a> Deref for CurrentGlContext<'a> {
    type Target = glutin::context::PossiblyCurrentContext;

    fn deref(&self) -> &Self::Target {
        self.current.as_ref().unwrap()
    }
}

impl<'a> Drop for CurrentGlContext<'a> {
    fn drop(&mut self) {
        self.not_current_gl_context
            .replace(self.current.take().unwrap().make_not_current().unwrap());
    }
}
