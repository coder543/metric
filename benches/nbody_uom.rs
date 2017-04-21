#![allow(non_snake_case, non_upper_case_globals)]

extern crate uom;

use self::uom::si::Quantity;
use self::uom::si::f64::*;
use self::uom::si::acceleration::meter_per_second_squared;
use self::uom::si::area::square_meter;
use self::uom::si::length::meter;
use self::uom::si::mass::kilogram;
use self::uom::si::time::second;
use self::uom::si::velocity::meter_per_second;
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
        // `sqrt` not yet implemented. https://github.com/iliekturtles/uom/issues/11
        (xd, yd, Length::new::<meter>((xd * xd + yd * yd).get(square_meter).sqrt()))
    }
}

#[inline(never)]
pub fn uom_nbody() {
    // Can't be `const` currently as `Quantity` members are private and constructor methods are not
    // `const fn`.
    let nbodies = [UOMNBody {
                       position: Position2D(Length::new::<meter>(1500.0), Length::new::<meter>(2500.0)),
                       accel: Accel2D(Acceleration::new::<meter_per_second_squared>(0.0),
                                      Acceleration::new::<meter_per_second_squared>(0.0)),
                       velocity: Velocity2D(Velocity::new::<meter_per_second>(0.0),
                                            Velocity::new::<meter_per_second>(0.0)),
                       mass: Mass::new::<kilogram>(2000.0),
                   },
                   UOMNBody {
                       position: Position2D(Length::new::<meter>(3500.0), Length::new::<meter>(500.0)),
                       accel: Accel2D(Acceleration::new::<meter_per_second_squared>(0.0),
                                      Acceleration::new::<meter_per_second_squared>(0.0)),
                       velocity: Velocity2D(Velocity::new::<meter_per_second>(0.0),
                                            Velocity::new::<meter_per_second>(0.0)),
                       mass: Mass::new::<kilogram>(2000.0),
                   },
                   UOMNBody {
                       position: Position2D(Length::new::<meter>(200.0), Length::new::<meter>(4500.0)),
                       accel: Accel2D(Acceleration::new::<meter_per_second_squared>(0.0),
                                      Acceleration::new::<meter_per_second_squared>(0.0)),
                       velocity: Velocity2D(Velocity::new::<meter_per_second>(0.0),
                                            Velocity::new::<meter_per_second>(0.0)),
                       mass: Mass::new::<kilogram>(2000.0),
                   },
                   UOMNBody {
                       position: Position2D(Length::new::<meter>(-1500.0), Length::new::<meter>(750.0)),
                       accel: Accel2D(Acceleration::new::<meter_per_second_squared>(0.0),
                                      Acceleration::new::<meter_per_second_squared>(0.0)),
                       velocity: Velocity2D(Velocity::new::<meter_per_second>(0.0),
                                            Velocity::new::<meter_per_second>(0.0)),
                       mass: Mass::new::<kilogram>(2000.0),
                   }];

    // Gravitational constant, unnamed quantity constructor still needs to be implemented.
    // https://github.com/iliekturtles/uom/issues/28
    let G: Quantity<uom::si::ISQ<P3, N1, N2, Z0, Z0, Z0, Z0>, uom::si::SI<f64>, f64> = unsafe { ::std::mem::transmute(6.674e-11) };
    let acceleration_zero: Acceleration = Acceleration::new::<meter_per_second_squared>(0.0);
    let time_step: Time = Time::new::<second>(0.1);

    let mut bodies = nbodies.to_vec();
    for _ in 0..10000 {
        //calculate accelerations
        for a in 0..bodies.len() {
            bodies[a].accel = Accel2D(acceleration_zero, acceleration_zero);
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
            bodies[a].velocity = Velocity2D(Vx + Ax * time_step, Vy + Ay * time_step);
            //integrate velocity into position
            let Velocity2D(Vx, Vy) = bodies[a].velocity;
            let Position2D(x, y) = bodies[a].position;
            bodies[a].position = Position2D(x + Vx * time_step, y + Vy * time_step);
        }
    }
}
