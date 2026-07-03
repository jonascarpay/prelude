use graphics::{color::oklch::Oklch, surface::vector::VectorSurface};
use prelude::algebra::{abstract_::Functor, geometric::complex::Complex, v2, V2};

// type R = Fixed<i32, 8>;
// type C = Complex<R>;
type R = f64;
type C = Complex<R>;

fn r(r: f64) -> R {
    r
}

fn c(s: f64, i: f64) -> C {
    C { s: r(s), xy: r(i) }
}

fn main() {
    const IMG_WIDTH: usize = 500;
    const IMG_HEIGHT: usize = 500;
    const DIMS: V2<usize> = v2(IMG_WIDTH, IMG_HEIGHT);

    let remap = todo!(); // remap2(zero()..DIMS.map(|u| u as R), v2(-1., -1.)..v2(1., 1.));

    let r1 = c(0.5, 0.5);
    let r2 = c(-0.5, -0.5);
    let poly = todo!(); // Quadratic::from_roots(one(), r1, r2).over_ring();

    let surf = VectorSurface::generate(DIMS, |v| {
        let v = remap.evaluate(v.map(|u| u as R));
        let x: C = v.into_complex();
        let y = poly.evaluate(x);
        let lch = Oklch {
            l: (-y.magnitude()).exp(),
            c: 0.3,
            h: y.argument(),
        };
        lch.to_srgb()
    });

    surf.write_ppm("out.ppm").unwrap();
}
