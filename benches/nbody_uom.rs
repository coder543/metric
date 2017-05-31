#![allow(non_snake_case, non_upper_case_globals)]

extern crate uom;

use std::marker::PhantomData;

use self::uom::si::Quantity;
use self::uom::si::f64::*;
use self::uom::typenum::{P3, N1, N2, Z0};

#[derive(Copy, Clone)]
struct UOMNBody {
    position: Position2D,
    accel: Accel2D,
    velocity: Velocity2D,
    mass: Mass,
}

#[derive(Copy, Clone, Debug)]
struct Accel2D(Acceleration, Acceleration);

#[derive(Copy, Clone, Debug)]
struct Velocity2D(Velocity, Velocity);

#[derive(Copy, Clone, Debug)]
struct Position2D(Length, Length);

impl Position2D {
    fn dist(&self, other: &Position2D) -> (Length, Length, Length) {
        let &Position2D(x1, y1) = self;
        let &Position2D(x2, y2) = other;
        let xd = x2 - x1;
        let yd = y2 - y1;
        (xd, yd, (xd * xd + yd * yd).sqrt())
    }
}

const NBODIES: [UOMNBody; 4] = [
    UOMNBody {
        position: Position2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 1500.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 2500.0, }),
        accel: Accel2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, }),
        velocity: Velocity2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, }),
        mass: Quantity { dimension: PhantomData, units: PhantomData, value: 2000.0, },
    },
    UOMNBody {
        position: Position2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 3500.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 500.0, }),
        accel: Accel2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, }),
        velocity: Velocity2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, }),
        mass: Quantity { dimension: PhantomData, units: PhantomData, value: 2000.0, },
    },
    UOMNBody {
        position: Position2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 200.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 4500.0, }),
        accel: Accel2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, }),
        velocity: Velocity2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, }),
        mass: Quantity { dimension: PhantomData, units: PhantomData, value: 2000.0, },
    },
    UOMNBody {
        position: Position2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: -1500.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 750.0, }),
        accel: Accel2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, }),
        velocity: Velocity2D(
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, },
            Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, }),
        mass: Quantity { dimension: PhantomData, units: PhantomData, value: 2000.0, },
    }];
// Gravitational constant.
const G: Quantity<uom::si::ISQ<P3, N1, N2, Z0, Z0, Z0, Z0>, uom::si::SI<f64>, f64> = Quantity { dimension: PhantomData, units: PhantomData, value: 6.674e-11, };
const ACCELERATION_ZERO: Acceleration = Quantity { dimension: PhantomData, units: PhantomData, value: 0.0, };
const TIME_STEP: Time = Quantity { dimension: PhantomData, units: PhantomData, value: 0.1, };

#[inline(never)]
pub fn uom_nbody() {
    let mut bodies = NBODIES.to_vec();
    for _ in 0..10000 {
        //calculate accelerations
        for a in 0..bodies.len() {
            bodies[a].accel = Accel2D(ACCELERATION_ZERO, ACCELERATION_ZERO);
            for b in 0..bodies.len() {
                if a == b {
                    continue;
                }
                let La = bodies[a].position;
                let Lb = bodies[b].position;
                let Ma = bodies[a].mass;
                let Mb = bodies[b].mass;
                let (Dx, Dy, dist) = La.dist(&Lb);
                let force = G / ((dist * dist) / (Ma * Mb));
                let Fx = force * (Dx / Dy);
                let Fy = force * (Dy / Dx);
                let Ax = Fx / Ma;
                let Ay = Fy / Ma;
                bodies[a].accel = Accel2D(bodies[a].accel.0 + Ax, bodies[a].accel.1 + Ay);
            }
        }
        for a in 0..bodies.len() {
            //integrate acceleration into velocity
            let Velocity2D(Vx, Vy) = bodies[a].velocity;
            let Accel2D(Ax, Ay) = bodies[a].accel;
            bodies[a].velocity = Velocity2D(Vx + Ax * TIME_STEP, Vy + Ay * TIME_STEP);
            //integrate velocity into position
            let Velocity2D(Vx, Vy) = bodies[a].velocity;
            let Position2D(x, y) = bodies[a].position;
            bodies[a].position = Position2D(x + Vx * TIME_STEP, y + Vy * TIME_STEP);
        }
    }
}
