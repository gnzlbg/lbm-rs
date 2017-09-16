use std::io::prelude::*;
use std::fs::File;
use std::fmt::Display;

use grid::*;
use num;

pub trait Primitive: Display {
    fn type_name() -> &'static str;
}

impl Primitive for f32 {
    fn type_name() -> &'static str {
        "float"
    }
}

impl Primitive for f64 {
    fn type_name() -> &'static str {
        "double"
    }
}


impl Primitive for i32 {
    fn type_name() -> &'static str {
        "int"
    }
}

pub struct CellDataWriter {
    buffer: File,
    grid: StructuredRectangular,
    init: bool,
}

impl CellDataWriter {
    pub fn new(buffer: File, grid: StructuredRectangular) -> CellDataWriter {
        CellDataWriter {
            buffer: buffer,
            grid: grid,
            init: false,
        }
    }

    pub fn write_scalar<T: Primitive, F: Fn(Idx) -> T>(&mut self,
                                                       name: &str,
                                                       f: F) {
        if !self.init {
            // Write cell data
            self.buffer
                .write(format!("CELL_DATA {}\n", self.grid.size()).as_bytes())
                .unwrap();
            self.init = true;
        }

        self.buffer
            .write(format!("SCALARS {} {}\n", name, T::type_name()).as_bytes())
            .unwrap();
        self.buffer.write(b"LOOKUP_TABLE default\n").unwrap();
        for c in self.grid.ids() {
            self.buffer.write(format!("{}\n", f(c)).as_bytes()).unwrap();
        }
    }
}

pub fn write_vtk(fname: &str, grid: StructuredRectangular) -> CellDataWriter {
    let mut buffer = File::create(format!("{}.vtk", fname)).unwrap();

    // Write Header
    buffer.write(b"# vtk DataFile Version 2.0\nLBM test output\nASCII\nDATASET UNSTRUCTURED_GRID\n"
    ).unwrap();

    let x_stencil = [-1., 1., -1., 1.];
    let y_stencil = [-1., -1., 1., 1.];
    let length = 1.;


    // Write grid points
    buffer
        .write(format!("POINTS {} FLOAT\n", grid.size() * 4).as_bytes())
        .unwrap();

    for c in grid.ids() {
        let X(x, y) = grid.x(c);
        for i in 0..4 {
            let xp = x as num + x_stencil[i] * 0.5 * length;
            let yp = y as num + y_stencil[i] * 0.5 * length;
            buffer
                .write(format!("{} {} 0.0\n", xp, yp).as_bytes())
                .unwrap();

        }
    }

    // Write grid cells:
    buffer
        .write(format!("CELLS {} {}\n", grid.size(), grid.size() * 5)
                   .as_bytes())
        .unwrap();
    for c in grid.ids() {
        buffer
            .write(format!("4 {} {} {} {}\n",
                           4 * c.0,
                           4 * c.0 + 1,
                           4 * c.0 + 2,
                           4 * c.0 + 3)
                           .as_bytes())
            .unwrap();
    }


    buffer
        .write(format!("CELL_TYPES {}\n", grid.size()).as_bytes())
        .unwrap();
    for _ in grid.ids() {
        buffer.write(b"8\n").unwrap();
    }

    CellDataWriter::new(buffer, grid)
}
