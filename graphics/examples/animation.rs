use std::{num::NonZeroU32, rc::Rc};

use graphics::buffer2d::Buffer2DView;
use prelude::{
    algebra::{
        abstract_::Curve,
        geometric::vec2::{v2, V2},
        polynomial::linear::{remap, Linear},
    },
    plot::ray::{plot_ray2d, Ray2D},
};
use softbuffer as sb;
use winit::{
    event_loop::{ActiveEventLoop, EventLoop},
    raw_window_handle::HasDisplayHandle,
    window::Window,
};

/// Maps buffer pixel coords (`0..w`, `0..h`, y-down) to normalized device
/// coordinates (`-1..1`, y-up). The map is axis-separable, so it's a `Linear`
/// per axis (`ndc = c1·coord + c0`); the y axis is flipped via a negative slope.
fn screen_to_ndc(w: f32, h: f32) -> V2<Linear<f32>> {
    v2(Linear { c1: 2.0 / w, c0: -1.0 }, Linear { c1: -2.0 / h, c0: 1.0 })
}

/// Inverse of [`screen_to_ndc`]: maps NDC (`-1..1`, y-up) back to buffer pixel
/// coords. Derived mechanically by inverting each axis, so it can never drift
/// out of sync with the forward map.
fn ndc_to_screen(w: f32, h: f32) -> V2<Linear<f32>> {
    screen_to_ndc(w, h).map(Linear::inverse)
}

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
                let size = view.size();
                let to_ndc = v2(
                    remap(0. ..size.x as f64, -1. ..1.), //
                    remap(0. ..size.y as f64, 1. ..-1.), //
                );
                // TODO Group so we can do .inverse here
                let to_screen = ndc_to_screen(size.x as f32, size.y as f32);
                // Define the ray in resolution-independent NDC, then map to pixels.
                let from_ndc = |x: f32, y: f32| {
                    let p = to_screen.evaluate(v2(x, y));
                    v2(p.x as usize, p.y as usize)
                };
                let mut ray: Ray2D<usize> = plot_ray2d(from_ndc(-0.9, -0.9), from_ndc(0.6, 0.3));
                for _ in 0..10000 {
                    let p = ray.step();
                    *view.get_mut(p.map(|i| i as usize)) = 0xFFFFFFFF;
                    let n = ray.peek();
                    // Bounce off the screen edges, expressed in NDC: the walls are
                    // at |x| = 1 and |y| = 1 regardless of resolution.
                    let ndc = to_ndc.evaluate(v2(n.x as f32, n.y as f32));
                    if ndc.x.abs() >= 1.0 {
                        ray.reflect_in_y();
                    }
                    if ndc.y.abs() >= 1.0 {
                        ray.reflect_in_x();
                    }
                }
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
