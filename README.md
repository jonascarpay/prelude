# prelude

Working title.

Don't use this library in any way.

This is (will be) a collection of the things I wish Rust had, or had better defaults for, designed to be
- pragmatic (make me more productive)
- simple (only things that are immediately self-explanatory (to me))
- fast (by being optimizer-friendly)
- coherent (a small, internally consistent set of traits and types)
- self-contained (only depends on `std`)

## Contents

Everything is incomplete and WIP, nothing is finished.
I'll be moving in more (mostly gamedev related/inspired) stuff as I go.
Currently contains:

```
- algebra
  - abstract algebra
    See below
  - geometric algebra
    Vectors and complex numbers, interpreted as members of geometric/Clifford algebras
  - linear algebra
    Matrices
  - numeric
    Fixed-precision arithmetic
  - polynomial
    splines, interpolations
- random
  RNGs and distributions
```

### Abstract Algebra

The algebraic traits are the backbone of the library.
They minimize boilerplate by collecting a maximally covering coherent set of methods.

Consider vector types.
It takes just four small methods to implement `Additive` and `VectorSpace`, and gets you:
  - vector algebra, including arithmetic operators through macros
  - (differentiable) interpolations and splines
  - iterator sums

#### Hierarchy

```
Additive (Abelian/additive groups)
  Ring
    EuclideanRing (integer division)
    Field
  VectorSpace
    InnerProductSpace

Semigroup
  Monoid
    Group

Curve
  DifferentiableCurve
```
