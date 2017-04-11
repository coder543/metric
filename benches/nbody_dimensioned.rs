#![allow(non_snake_case, non_upper_case_globals)]

extern crate dimensioned as dim;

use self::dim::si;

#[derive(Copy, Clone)]
struct MetricNBody {
    position: Position2D,
    accel: Accel2D,
    velocity: Velocity2D,
    mass: si::Kilogram<f64>,
}

use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
struct Accel2D(si::MeterPerSecond2<f64>, si::MeterPerSecond2<f64>);
const MPSS_ZERO: si::MeterPerSecond2<f64> = si::MeterPerSecond2 { value_unsafe: 0.0, _marker: PhantomData };

#[derive(Copy, Clone, Debug)]
struct Velocity2D(si::MeterPerSecond<f64>, si::MeterPerSecond<f64>);
const MPS_ZERO: si::MeterPerSecond<f64> = si::MeterPerSecond { value_unsafe: 0.0, _marker: PhantomData };

#[derive(Copy, Clone, Debug)]
struct Position2D(si::Meter<f64>, si::Meter<f64>);

impl Position2D {
    fn dist(&self, other: &Position2D) -> (si::Meter<f64>, si::Meter<f64>, si::Meter<f64>) {
        let &Position2D(x1, y1) = self;
        let &Position2D(x2, y2) = other;
        let xd = x2 - x1;
        let yd = y2 - y1;
        (xd, yd, (xd * xd + yd * yd).value_unsafe.sqrt() * si::M)
    }
}

const MetricNBodies: [MetricNBody; 4] = [MetricNBody {
                                             position: Position2D(si::Meter { value_unsafe: 1500.0, _marker: PhantomData }, si::Meter { value_unsafe: 2500.0, _marker: PhantomData }),
                                             accel: Accel2D(MPSS_ZERO, MPSS_ZERO),
                                             velocity: Velocity2D(MPS_ZERO, MPS_ZERO),
                                             mass: si::Kilogram { value_unsafe: 2000.0, _marker: PhantomData }
                                         },
                                         MetricNBody {
                                             position: Position2D(si::Meter { value_unsafe: 3500.0, _marker: PhantomData }, si::Meter { value_unsafe: 500.0, _marker: PhantomData }),
                                             accel: Accel2D(MPSS_ZERO, MPSS_ZERO),
                                             velocity: Velocity2D(MPS_ZERO, MPS_ZERO),
                                             mass: si::Kilogram { value_unsafe: 2000.0, _marker: PhantomData }
                                         },
                                         MetricNBody {
                                             position: Position2D(si::Meter { value_unsafe: 200.0, _marker: PhantomData }, si::Meter { value_unsafe: 4500.0, _marker: PhantomData }),
                                             accel: Accel2D(MPSS_ZERO, MPSS_ZERO),
                                             velocity: Velocity2D(MPS_ZERO, MPS_ZERO),
                                             mass: si::Kilogram { value_unsafe: 2000.0, _marker: PhantomData }
                                         },
                                         MetricNBody {
                                             position: Position2D(si::Meter { value_unsafe: -1500.0, _marker: PhantomData }, si::Meter { value_unsafe: 750.0, _marker: PhantomData }),
                                             accel: Accel2D(MPSS_ZERO, MPSS_ZERO),
                                             velocity: Velocity2D(MPS_ZERO, MPS_ZERO),
                                             mass: si::Kilogram { value_unsafe: 2000.0, _marker: PhantomData }
                                         }];

#[inline(never)]
pub fn dimensioned_nbody() {
    let G = 6.674e-11 * si::KG * si::M * si::M / (si::KG * si::KG);
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

                //even confirming that `dist` *is* a Meter, we cannot successfully
                //multiply a Meter against itself to get a Meter^2.
                let dist: si::Meter<f64> = dist;
                let di2: si::Meter2<f64> = dist * dist;

                let ma2 = Ma * Mb; //can't constrain because Kilogram2 does not exist

                //because of the above problems, there's absolutely no way this line compiles:
                let force: si::Newton<f64> = G * ma2 / di2;
                
                let Fx: si::Newton<f64> = force * (Dx / Dy);
                let Fy: si::Newton<f64> = force * (Dy / Dx);
                let Ax: si::MeterPerSecond2<f64> = Fx / Ma;
                let Ay: si::MeterPerSecond2<f64> = Fy / Ma;
                bodies[a].accel = Accel2D(bodies[a].accel.0 + Ax, bodies[a].accel.1 + Ay);
            }
        }
        for a in 0..bodies.len() {
            //integrate acceleration into velocity
            let Velocity2D(Vx, Vy) = bodies[a].velocity;
            let Accel2D(Ax, Ay) = bodies[a].accel;
            let Vx1: si::MeterPerSecond<f64> = Ax * (0.1 * si::S);
            let Vy1: si::MeterPerSecond<f64> = Ay * (0.1 * si::S);
            bodies[a].velocity = Velocity2D(Vx + Vx1, Vy + Vy1);
            //integrate velocity into position
            let Velocity2D(Vx, Vy) = bodies[a].velocity;
            let Position2D(x, y) = bodies[a].position;
            bodies[a].position = Position2D(x + Vx * (0.1 * si::S), y + Vy * (0.1 * si::S));
        }
    }
}
