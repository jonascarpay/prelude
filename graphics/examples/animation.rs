use std::{num::NonZeroU32, rc::Rc};

use graphics::buffer2d::Buffer2D;
use prelude::{
    algebra::{
        abstract_::{additive::step_by_until, field::Field, Additive, Curve, Group, VectorSpace},
        geometric::vec2::{v2, V2},
        numeric::fixed::Fixed,
        polynomial::{
            cubic::{self, bezier3, Cubic},
            linear::{remap2, Linear},
        },
        Ring,
    },
    plot::{itertools::bigrams, line::plot_line2d},
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
    v2(
        Linear {
            c1: 2.0 / w,
            c0: -1.0,
        },
        Linear {
            c1: -2.0 / h,
            c0: 1.0,
        },
    )
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
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
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
                let sctx = self
                    .surface_context
                    .as_mut()
                    .expect("Resize without surface context");
                let width = NonZeroU32::new(width).expect("Zero surface width");
                let height = NonZeroU32::new(height).expect("Zero surface height");
                sctx.surface
                    .resize(width, height)
                    .expect("Error while resizing");
            }
            RedrawRequested => {
                let sctx = self
                    .surface_context
                    .as_mut()
                    .expect("Redraw without surface context");
                let mut buf = sctx.surface.buffer_mut().unwrap();
                let mut view = Buffer2D::from_softbuffer(&mut buf);
                let size = view.size();

                type R = Fixed<i64, 8>;
                let one = R::one();
                let two = one + one;
                let zero = R::zero();
                let half = two.recip();
                fn r(x: f64) -> R {
                    R::from_f64(x)
                }
                fn vu(x: V2<R>) -> V2<usize> {
                    v2(x.x.trunc() as usize, x.y.trunc() as usize)
                }

                let size_f: V2<R> = v2(
                    R::from_integer(size.x as isize),
                    R::from_integer(size.y as isize),
                );

                let ndc_to_screen_map =
                    remap2(v2(-one, one)..v2(one, -one), v2(zero, zero)..size_f);
                let screen_to_ndc_map = ndc_to_screen_map.inverse();

                let clip = v2(zero, zero)..size_f;

                // let mut draw_line = |start: V2<R>, end: V2<R>| {
                //     let start = ndc_to_screen_map.evaluate(start);
                //     let end = ndc_to_screen_map.evaluate(end);
                //     if start.in_bounds(V2::zero()..size_f) && end.in_bounds(V2::zero()..size_f) {
                //         for p in plot_line2d(vu(start), vu(end)) {
                //             *view.get_mut(p) = 0x00FFFFFF;
                //         }
                //     }
                // };
                let mut draw_line = |start: V2<R>, end: V2<R>| {
                    let start = ndc_to_screen_map.evaluate(start);
                    let end = ndc_to_screen_map.evaluate(end);
                    if start.in_bounds(V2::zero()..size_f) && end.in_bounds(V2::zero()..size_f) {
                        // *view.get_mut(vu(start)) = 0x00FFFFFF;
                        for p in plot_line2d(vu(start), vu(end)) {
                            *view.get_mut(p) = 0x00FFFFFF;
                        }
                    }
                };

                let x0: Cubic<R> = Cubic::x0();
                let x1: Cubic<R> = Cubic::x1();
                let x2: Cubic<R> = Cubic::x2();
                let x3: Cubic<R> = Cubic::x3();
                let curves = [
                    // x0,
                    // x1,
                    // x2,
                    // x3,
                    Cubic::from_roots(r(2.0), -r(0.5), r(0.), r(0.5)),
                    // Cubic::from_roots(one, -r(0.5), r(0.), r(0.5)) + x0 * R::EPSILON,
                ];

                let b = bezier3(
                    v2(-half, half),
                    v2(one, one),
                    v2(-one, -one),
                    v2(half, -half),
                );

                for (xa, xb) in bigrams(step_by_until(-one, R::EPSILON, one)) {
                    for curve in curves {
                        let va = v2(xa, curve.evaluate(xa));
                        let vb = v2(xb, curve.evaluate(xb));
                        draw_line(va, vb);
                    }
                }
                for (xa, xb) in bigrams(step_by_until(r(-1.0), R::EPSILON, r(2.0))) {
                    draw_line(b.evaluate(xa), b.evaluate(xb));
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
