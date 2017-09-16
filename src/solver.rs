//! Implements the Lattice-Boltzmann solver.

use num;
use grid;
use boundary;
use io::{vtk, Serializable};
use time;
use traits::{Distribution, DistributionStorage};

/// Lattice-Boltzmann Solver state
pub struct Solver<P: ::Physics> {
    grid: grid::StructuredRectangular,
    pub bcs: boundary::Handler,
    physics: P,
    f: Box<[num]>, // Distribution functions
    f_hlp: Box<[num]>,
}

impl<P: ::Physics> Solver<P> {
    /// Create a new solver from a `grid` and `physics`.
    pub fn new(grid: grid::StructuredRectangular, physics: P) -> Solver<P> {
        Solver {
            grid,
            bcs: boundary::Handler::default(),
            physics,
            f: vec![0.; grid.size() * P::Distribution::size()]
                .into_boxed_slice(),
            f_hlp: vec![0.; grid.size() * P::Distribution::size()]
                .into_boxed_slice(),
        }
    }

    /// Initialize distributions functions using `initial_distributions(x)`.
    pub fn initialize<F>(&mut self, initial_distributions: F)
    where
        F: Fn(grid::X)
            -> DistributionStorage<P::Distribution>,
    {
        for c in self.grid.ids() {
            let fs = initial_distributions(self.grid.x(c));
            for n in P::Distribution::all() {
                *self.f_mut(c, n) = fs.as_ref()[n.value()];
            }
        }
    }

    /// Index of the distribution function `i` of the cell `c`
    fn f_idx(c: grid::Idx, i: P::Distribution) -> usize {
        c.0 * P::Distribution::size() + i.value()
    }

    /// Mutable reference to the distribution function `i` of the cell `c`
    fn f_mut(&mut self, c: grid::Idx, i: P::Distribution) -> &mut num {
        &mut self.f[Self::f_idx(c, i)]
    }

    /// Reference to the distribution function `i` of the cell `c`
    fn f_ref(&self, c: grid::Idx, i: P::Distribution) -> &num {
        &self.f[Self::f_idx(c, i)]
    }

    /// Is the cell `c` par of a solid boundary?
    fn solid_boundary(&self, c: grid::Idx) -> bool {
        self.bcs.solid_boundary(self.grid.x(c))
    }

    /// Streaming step
    fn streaming(&mut self) {
        use rayon::prelude::*;
        let mut f_hlp =
            ::std::mem::replace(&mut self.f_hlp, Default::default());
        f_hlp
            .par_chunks_mut(P::Distribution::size())
            .zip(self.grid.par_ids())
            .for_each(|(f_hlp, c)| for n in P::Distribution::all() {
                let cn = self.grid.neighbor(c, n);
                f_hlp[n.value()] = *self.f_ref(cn, n);
            });
        self.f_hlp = f_hlp;
    }

    /// Collision step
    fn collision(&mut self) {
        let mut f = ::std::mem::replace(&mut self.f, Default::default());
        use rayon::prelude::*;
        f.par_chunks_mut(P::Distribution::size())
            .zip(self.f_hlp.par_chunks(P::Distribution::size()))
            .zip(self.grid.par_ids())
            .for_each(|((f, f_hlp), c)| {
                if self.solid_boundary(c) {
                    return;
                }
                let r = self.physics.collision(&f_hlp, |v, n| v[n.value()]);

                for n in P::Distribution::all() {
                    f[n.value()] = r.as_ref()[n.value()];
                }
            });
        self.f = f;
    }

    /// Applies boundary conditions
    fn apply_boundary_conditions(&mut self) {
        let mut f = ::std::mem::replace(&mut self.f, Default::default());
        use rayon::prelude::*;
        f.par_chunks_mut(P::Distribution::size())
            .zip(self.grid.par_ids())
            .for_each(|(f, c)| {
                let r = self.bcs.apply(
                    &f,
                    &self.f_hlp,
                    |v, n: P::Distribution| v[n.value()],
                    |v, n| v[Self::f_idx(c, n)],
                    self.grid.x(c),
                );

                if let Some(r) = r {
                    for n in P::Distribution::all() {
                        f[n.value()] = r.as_ref()[n.value()];
                    }
                }
            });
        self.f = f;
    }

    /// Executes `n_it` iterations writing output every `n_out` iterations.
    pub fn run(&mut self, n_it: usize, n_out: usize) {
        let mut n_it = n_it;
        assert!(n_it > 0);
        let mut iter = 0;
        use time::Duration;

        loop {
            let write_output = n_out > 0 && iter % n_out == 0;
            let d = Duration::span(|| {
                let d = Duration::span(|| self.streaming());
                if write_output {
                    self.substep("propagation", d);
                }

                let d = Duration::span(|| self.collision());
                if write_output {
                    self.substep("collision", d);
                }

                let d = Duration::span(|| self.apply_boundary_conditions());
                if write_output {
                    self.substep("bcs", d);
                }

                n_it -= 1;
                if write_output {
                    let d = Duration::span(|| self.write_vtk(iter));
                    self.substep("vtk", d);
                }
            });
            if write_output {
                self.step(iter, d);
            }
            if n_it == 0 {
                break;
            }
            iter += 1;
        }
    }

    /// Integrates the distribution functions over the volume
    fn integral(&self) -> num {
        use rayon::prelude::*;
        self.grid
            .par_ids()
            .map(|c| P::integral(|n| *self.f_ref(c, n)))
            .sum()
    }

    /// Prints line info of a whole iteration step
    fn step(&self, n_it: usize, duration: time::Duration) {
        let integral = self.integral();
        println!(
            "#{} | integral: {} | duration: {} ms",
            n_it,
            integral,
            duration.num_milliseconds()
        );
    }
    /// Prints line info of an iteration sub-step
    fn substep(&self, name: &str, duration: time::Duration) {
        let res = self.integral();
        println!(
            "# [{}] | integral: {} | duration: {} \u{03BC}s",
            name,
            res,
            duration.num_microseconds().unwrap()
        );
    }

    /// Writes the solution to a VTK file.
    fn write_vtk(&self, n_it: usize) {
        let fname = format!("lbm_rs_output_{}", n_it);
        let mut vtk_writer = vtk::write_vtk(&fname, self.grid);
        vtk_writer = self.physics.write(
            vtk_writer,
            |c| self.solid_boundary(c),
            |c, n| *self.f_ref(c, n),
        );
        vtk_writer.write_scalar("boundary_idx", |c| {
            self.bcs.idx(self.grid.x(c)).map_or(-1 as i32, |v| v as i32)
        });
    }
}

/*
impl<P: ::Physics> Serializable for Solver<P> {
    type CellIndex = grid::Idx;
    type CellGeometry = geometry::Square;
    type CellIndexIterator = _;
    fn cells(&self) -> {
        self.grid.ids()
    }
    fn geometry(&self, c: Self::CellIndex) -> Self::CellGeometry {
        self.grid.geometry(c)
    }
    fn cell_data<T: CellDataWriter<Self::CellIndex>>(&self, writer: &mut T) {
        self.physics.write(
            writer,
            |c| self.solid_boundary(c),
            |c, n| *self.f_ref(c, n),
        );
        writer.write_scalar("boundary_idx", |c| {
            self.bcs.idx(self.grid.x(c)).map_or(-1 as i32, |v| v as i32)
        });
    }
}
*/
