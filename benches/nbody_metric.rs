#![allow(non_snake_case, non_upper_case_globals)]

extern crate metric;

use self::metric::time::Second;
use self::metric::force::Newton;
use self::metric::length::metric::Meter;
use self::metric::mass::metric::Kilogram;
use self::metric::composite::*;


#[derive(Copy, Clone)]
struct MetricNBody {
    position: Position2D,
    accel: Accel2D,
    velocity: Velocity2D,
    mass: Kilogram,
}

use std::marker::PhantomData;

///Meter per Second^2
type MPSS = Div<Meter, Mul<Second, Second>>;
const MPSS_ZERO: MPSS = Div(Meter(0.0), PhantomData);

#[derive(Copy, Clone, Debug)]
struct Accel2D(MPSS, MPSS);

///Meter per Second
type MPS = Div<Meter, Second>;
const MPS_ZERO: MPS = Div(Meter(0.0), PhantomData);

#[derive(Copy, Clone, Debug)]
struct Velocity2D(MPS, MPS);

#[derive(Copy, Clone, Debug)]
struct Position2D(Meter, Meter);

impl Position2D {
    fn dist(&self, other: &Position2D) -> (Meter, Meter, Meter) {
        let &Position2D(x1, y1) = self;
        let &Position2D(x2, y2) = other;
        let xd = x2 - x1;
        let yd = y2 - y1;
        (xd, yd, (xd*xd + yd*yd).sqrt())
    }
}

const MetricNBodies: [MetricNBody; 4] = [MetricNBody {
                                       position: Position2D(Meter(1500.0), Meter(2500.0)),
                                       accel: Accel2D(MPSS_ZERO, MPSS_ZERO),
                                       velocity: Velocity2D(MPS_ZERO, MPS_ZERO),
                                       mass: Kilogram(2000.0),
                                   },
                                   MetricNBody {
                                       position: Position2D(Meter(3500.0), Meter(500.0)),
                                       accel: Accel2D(MPSS_ZERO, MPSS_ZERO),
                                       velocity: Velocity2D(MPS_ZERO, MPS_ZERO),
                                       mass: Kilogram(2000.0),
                                   },
                                   MetricNBody {
                                       position: Position2D(Meter(200.0), Meter(4500.0)),
                                       accel: Accel2D(MPSS_ZERO, MPSS_ZERO),
                                       velocity: Velocity2D(MPS_ZERO, MPS_ZERO),
                                       mass: Kilogram(2000.0),
                                   },
                                   MetricNBody {
                                       position: Position2D(Meter(-1500.0), Meter(750.0)),
                                       accel: Accel2D(MPSS_ZERO, MPSS_ZERO),
                                       velocity: Velocity2D(MPS_ZERO, MPS_ZERO),
                                       mass: Kilogram(2000.0),
                                   }];

lazy_static! {
    static ref G: Mul<Newton, Mul<Meter, Div<Meter, Mul<Kilogram, Kilogram>>>> = Mul(Newton::new(6.674e-11), PhantomData);
}

pub fn metric_nbody() {
    let mut bodies = MetricNBodies.to_vec();
    for _ in 0..10000 {
        //calculate accelerations
        for a in 0..bodies.len() {
            bodies[a].accel = Accel2D(MPSS_ZERO, MPSS_ZERO);
            for b in 0..bodies.len() {
                if a == b {
                    continue;
                }
                let La = bodies[a].position;
                let Lb = bodies[b].position;
                let Ma = bodies[a].mass;
                let Mb = bodies[b].mass;
                let (Dx, Dy, dist) = La.dist(&Lb);
                let force: Newton = G.divide_right((dist * dist)/(Ma * Mb));
                let Fx: Newton = force * (Dx / Dy);
                let Fy: Newton = force * (Dy / Dx);
                let Ax: MPSS = Fx.divide_left(Ma);
                let Ay: MPSS = Fy.divide_left(Ma);
                bodies[a].accel = Accel2D(bodies[a].accel.0 + Ax, bodies[a].accel.1 + Ay);
            }
        }
        for a in 0..bodies.len() {
            //integrate acceleration into velocity
            let Velocity2D(Vx, Vy) = bodies[a].velocity;
            let Accel2D(Ax, Ay) = bodies[a].accel;
            let Vx1: MPS = Ax.0 / Second(0.1);
            let Vy1: MPS = Ay.0 / Second(0.1);
            bodies[a].velocity = Velocity2D(Vx + Vx1, Vy + Vy1);
            //integrate velocity into position
            let Velocity2D(Vx, Vy) = bodies[a].velocity;
            let Position2D(x, y) = bodies[a].position;
            bodies[a].position = Position2D(x + Vx.multiply(Second(0.1)), y + Vy.multiply(Second(0.1)));
        }
    }
}