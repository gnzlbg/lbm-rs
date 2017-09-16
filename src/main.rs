extern crate lbm;

use lbm::*;

// Configure the numerical method
type Dist = distribution::D2Q9;
type Col = physics::ns::SingleRelaxationTime;
type Physics = physics::NavierStokes<Dist, Col>;

fn main() {
    // Initialize the grid, physical parameters, and solver:
    let grid = grid::StructuredRectangular { x: 300, y: 150 };
    let physics: Physics = Physics::new(0.1, 0.015, Col { omega: 1.85 });
    let mut s = lbm::Solver::new(grid, physics);

    // Add Boundary Conditions:
    {
        // Cylinder:
        {
            let cyl = Box::new(boundary::Condition::new(
                boundary::Type::BounceBack,
                geometry::Circle::new(grid.x, grid.y),
            ));
            s.bcs.push(cyl);
        }
        // Bottom channel wall:
        {
            let bottom_wall = Box::new(boundary::Condition::new(
                boundary::Type::BounceBack,
                geometry::Plane::new((0, 1), (0, 0)),
            ));
            s.bcs.push(bottom_wall);
        }
        // Top channel wall:
        {
            let top_wall = Box::new(boundary::Condition::new(
                boundary::Type::BounceBack,
                geometry::Plane::new((0, -1), (0, grid.y - 1)),
            ));
            s.bcs.push(top_wall);
        }
        // Periodic forced inflow:
        {
            let bc = Box::new(boundary::Condition::new(
                boundary::Type::Inflow(
                    physics.inflow_density,
                    physics.inflow_accel,
                ),
                geometry::Plane::new((1, 0), (0, 0)),
            ));
            s.bcs.push(bc);
        }
    }

    // Initialize distribution functions
    s.initialize(|_| {
        let mut ns = DistributionStorage::<Dist>::default();
        for n in Dist::all() {
            ns.as_mut()[n.value()] = physics.inflow_density * n.constant();
        }
        ns
    });

    s.run(10001, 500);
}
