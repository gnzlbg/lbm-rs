//! Navier-Stokes physics
use std;
use num;
use io::vtk;
use grid;
use distribution;

/// Navier-Stokes distributions:
pub trait Distribution: ::DirectDistribution + ::DiagonalDistribution {
    #[inline(always)]
    fn density<F: Fn(Self) -> num>(f: F) -> num {
        let mut rho = 0.;
        for n in Self::all() {
            rho += f(n);
        }
        rho
    }

    #[inline(always)]
    fn pressure<F: Fn(Self) -> num>(f: F) -> num {
        Self::density(f) * Self::c_squ()
    }

    #[inline(always)]
    fn velocity<F: Fn(Self) -> num>(f: F, d: usize) -> num {
        let mut tmp = 0.;
        for n in Self::all() {
            tmp += n.direction().num_array()[d] * f(n);
        }
        tmp / Self::density(f)
    }

    #[inline(always)]
    fn velocities<F: Fn(Self) -> num>(f: F) -> [num; 2] {
        [Self::velocity(&f, 0), Self::velocity(&f, 1)]
    }
}

impl Distribution for distribution::D2Q9 {}

/// Single relaxation time (SRT) algorithm
#[derive(Copy, Clone)]
pub struct SingleRelaxationTime {
    pub omega: num,
}

impl<D: Distribution> ::Collision<D> for SingleRelaxationTime {
    #[inline(always)]
    fn collision<H, IH>(&self, f_hlp: &H, idx_h: IH) -> D::Storage
    where
        IH: Fn(&H, D) -> num,
    {
        // local density and vel components:
        let f_h = |n| idx_h(f_hlp, n);
        let dloc = D::density(&f_h);
        let [u_x, u_y] = D::velocities(&f_h);

        // n- velocity compnents (n = grid node connection vectors)
        // TODO: switch to 3 speeds only
        let mut u_n_ = D::Storage::default();
        {
            let u_n = u_n_.as_mut();
            for n in D::all() {
                let v = n.direction().num_array();
                let n = n.value();
                u_n[n] = v[0] * u_x + v[1] * u_y;
            }

            // equilibrium densities:
            let f0 = 2. * D::c_squ() * D::c_squ();
            let f1 = 2. * D::c_squ();
            let u_squ = u_x.powf(2.) + u_y.powf(2.); // square velocity
            let f2 = u_squ / f1;

            let mut n_equ_ = D::Storage::default();
            let n_equ = n_equ_.as_mut();

            // zero-th velocity density
            n_equ[0] = D::center().constant() * dloc * (1. - f2);

            for n in D::direct() {
                let f3 = n.constant() * dloc;
                let n = n.value();
                n_equ[n] =
                    f3 * (1. + u_n[n] / D::c_squ() + u_n[n].powf(2.) / f0 - f2);
            }
            for n in D::diagonal() {
                let f4 = n.constant() * dloc;
                let n = n.value();
                n_equ[n] =
                    f4 * (1. + u_n[n] / D::c_squ() + u_n[n].powf(2.) / f0 - f2);
            }

            // relaxation step:
            for n in D::all() {
                u_n[n.value()] =
                    f_h(n) + self.omega * (n_equ[n.value()] - f_h(n));
            }
        }
        u_n_
    }
}

#[derive(Copy, Clone)]
pub struct NavierStokes<D: Distribution, C: ::Collision<D>> {
    pub inflow_density: num,
    pub inflow_accel: num,
    collision: C,
    __dist: std::marker::PhantomData<D>,
}


impl<D: Distribution, C: ::Collision<D>> NavierStokes<D, C> {
    pub fn new(density: num, accel: num, col: C) -> Self {
        Self {
            inflow_density: density,
            inflow_accel: accel,
            collision: col,
            __dist: std::marker::PhantomData {},
        }
    }
    #[inline(always)]
    pub fn pressure<F: Fn(D) -> num>(&self, solid: bool, f: F) -> num {
        if solid {
            self.inflow_density * D::c_squ()
        } else {
            D::pressure(f)
        }
    }
    #[inline(always)]
    pub fn velocities<F: Fn(D) -> num>(solid: bool, f: F) -> [num; 2] {
        if solid {
            [0., 0.]
        } else {
            D::velocities(f)
        }
    }
}

impl<D: Distribution, C: ::Collision<D>> ::traits::Physics
    for NavierStokes<D, C> {
    type Distribution = D;
    #[inline(always)]
    fn collision<H, IH>(&self, f_hlp: &H, idx_h: IH) -> D::Storage
    where
        IH: Fn(&H, D) -> num,
    {
        self.collision.collision(f_hlp, idx_h)
    }
    #[inline(always)]
    fn integral<F: Fn(D) -> num>(f: F) -> num {
        D::density(f)
    }

    fn write<O, F>(
        &self,
        mut vtk_writer: vtk::CellDataWriter,
        obst: O,
        f: F,
    ) -> vtk::CellDataWriter
    where
        F: Fn(grid::Idx, D) -> num,
        O: Fn(grid::Idx) -> bool,
    {
        vtk_writer.write_scalar("p", |c| self.pressure(obst(c), |n| f(c, n)));
        vtk_writer
            .write_scalar("u", |c| Self::velocities(obst(c), |n| f(c, n))[0]);
        vtk_writer
            .write_scalar("v", |c| Self::velocities(obst(c), |n| f(c, n))[1]);
        vtk_writer
    }
}
