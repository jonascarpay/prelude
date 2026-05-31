use std::{num::NonZeroU32, rc::Rc};

use graphics::buffer2d::Buffer2DView;
use prelude::algebra::geometric::vec2::v2;
use softbuffer as sb;
use winit::{
    event_loop::{ActiveEventLoop, EventLoop},
    raw_window_handle::HasDisplayHandle,
    window::Window,
};

struct App<D> {
    context: sb::Context<D>,
    surface_context: Option<SurfaceContext<D>>,
}

struct SurfaceContext<D> {
    window: Rc<Window>,
    surface: sb::Surface<D, Rc<Window>>,
}

impl<D> App<D> {
    fn new(ctx: sb::Context<D>) -> Self {
        App {
            context: ctx,
            surface_context: None,
        }
    }
}

impl<D: HasDisplayHandle> winit::application::ApplicationHandler for App<D> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        let window = Rc::new(window);
        let surf = sb::Surface::new(&self.context, window.clone()).unwrap();
        self.surface_context = Some(SurfaceContext {
            window: window,
            surface: surf,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use winit::{dpi::PhysicalSize, event::WindowEvent::*};
        match event {
            Resized(PhysicalSize { width, height }) => {
                let sctx = self.surface_context.as_mut().expect("Resize without surface context");
                let width = NonZeroU32::new(width).expect("Zero surface width");
                let height = NonZeroU32::new(height).expect("Zero surface height");
                sctx.surface.resize(width, height).expect("Error while resizing");
            }
            RedrawRequested => {
                let sctx = self.surface_context.as_mut().expect("Redraw without surface context");
                let mut buf = sctx.surface.buffer_mut().unwrap();
                let mut view = Buffer2DView::from_softbuffer(&mut buf);
                *view.get_mut(v2(10, 20)) = 0xFFFFFFFF;
                buf.present().expect("Error presenting buffer");
            }
            _ => {
                dbg!(event);
            }
        };
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let context = sb::Context::new(event_loop.owned_display_handle()).unwrap();

    let mut app = App::new(context);

    event_loop.run_app(&mut app).unwrap();
}
