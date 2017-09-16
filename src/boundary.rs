use geometry::Geometry;
use grid;
use num;

#[derive(Copy,Clone,PartialEq,PartialOrd)]
pub enum Type {
    BounceBack,
    Inflow(num, num),
}

pub trait AnyCondition: Send + Sync {
    #[inline(always)]
    fn condition(&self) -> Type;
    #[inline(always)]
    fn contains(&self, grid::X) -> bool;
}

pub struct Condition<T: Geometry + Send + Sync> {
    condition: Type,
    geometry: T,
}

impl<T: Geometry + Send + Sync> Condition<T> {
    pub fn new(c: Type, g: T) -> Condition<T> {
        Condition {
            condition: c,
            geometry: g,
        }
    }
}

impl<T: Geometry + Send + Sync> AnyCondition for Condition<T> {
    #[inline(always)]
    fn condition(&self) -> Type {
        self.condition
    }
    #[inline(always)]
    fn contains(&self, x: grid::X) -> bool {
        self.geometry.contains(x)
    }
}

pub struct Handler {
    boundary_conditions: Vec<Box<AnyCondition>>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler { boundary_conditions: Vec::new() }
    }
    pub fn push(&mut self, bc: Box<AnyCondition>) {
        self.boundary_conditions.push(bc);
    }

    #[inline(always)]
    pub fn solid_boundary(&self, x: grid::X) -> bool {
        for bc in &self.boundary_conditions {
            if bc.contains(x) && bc.condition() == Type::BounceBack {
                return true;
            }
        }
        false
    }

    #[inline(always)]
    pub fn idx(&self, x: grid::X) -> Option<usize> {
        let mut idx = 0;
        for bc in &self.boundary_conditions {
            if bc.contains(x) {
                return Some(idx);
            }
            idx += 1;
        }
        None
    }

    #[inline(always)]
    pub fn apply<F, H, IF, IH, D>(&self,
                                  f: &F,
                                  f_hlp: &H,
                                  idx_f: IF,
                                  idx_h: IH,
                                  x: grid::X)
                                  -> Option<D::Storage>
        where IF: Fn(&F, D) -> num,
              IH: Fn(&H, D) -> num,
              D: ::Distribution
    {
        let mut r: Option<D::Storage> = None;

        for bc in &self.boundary_conditions {
            if !bc.contains(x) {
                continue;
            }
            match bc.condition() {
                Type::BounceBack => {
                    let mut s = D::Storage::default();
                    for n in D::all() {
                        s.as_mut()[n.value()] = idx_h(f_hlp, n.opposite());
                    }
                    r = Some(s);
                }
                Type::Inflow(density, accel) => {
                    let mut s_ = match r {
                        Some(s) => s,
                        None => {
                            let mut s = D::Storage::default();
                            for n in D::all() {
                                s.as_mut()[n.value()] = idx_f(f, n);
                            }
                            s
                        }
                    };
                    {
                        let s = s_.as_mut();

                        use geometry::Direction::*;
                        for n in D::all() {
                            let t = density * accel * n.constant();
                            match n.direction() {
                                W => {
                                    if s[D::from_direction(W)
                                           .unwrap()
                                           .value()] -
                                       t >
                                       0. {
                                        s[D::from_direction(E)
                                            .unwrap()
                                            .value()] += t;
                                        s[D::from_direction(W)
                                            .unwrap()
                                            .value()] -= t;
                                    }
                                }
                                NW => {
                                    if s[D::from_direction(NW)
                                           .unwrap()
                                           .value()] -
                                       t >
                                       0. {
                                        s[D::from_direction(SE)
                                            .unwrap()
                                            .value()] += t;
                                        s[D::from_direction(NW)
                                            .unwrap()
                                            .value()] -= t;
                                    }
                                }
                                SW => {
                                    if s[D::from_direction(SW)
                                           .unwrap()
                                           .value()] -
                                       t >
                                       0. {
                                        s[D::from_direction(NE)
                                            .unwrap()
                                            .value()] += t;
                                        s[D::from_direction(SW)
                                            .unwrap()
                                            .value()] -= t;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    r = Some(s_);
                }
            }
        }
        r
    }
}
