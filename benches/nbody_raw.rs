#![allow(non_snake_case, non_upper_case_globals)]

const G_RAW: f64 = 6.674e-11;

#[derive(Copy, Clone)]
struct RawNBody {
    position: Raw2D,
    accel: Raw2D,
    velocity: Raw2D,
    mass: f64,
}

#[derive(Copy, Clone, Debug)]
struct Raw2D(f64, f64);

impl Raw2D {
    fn dist(&self, other: &Raw2D) -> (f64, f64, f64) {
        let &Raw2D(x1, y1) = self;
        let &Raw2D(x2, y2) = other;
        let xd = x2 - x1;
        let yd = y2 - y1;
        (xd, yd, (xd * xd + yd * yd).sqrt())
    }
}

const RawNBodies: [RawNBody; 4] = [RawNBody {
                                       position: Raw2D(1500.0, 2500.0),
                                       accel: Raw2D(0.0, 0.0),
                                       velocity: Raw2D(0.0, 0.0),
                                       mass: 2000.0,
                                   },
                                   RawNBody {
                                       position: Raw2D(3500.0, 500.0),
                                       accel: Raw2D(0.0, 0.0),
                                       velocity: Raw2D(0.0, 0.0),
                                       mass: 2000.0,
                                   },
                                   RawNBody {
                                       position: Raw2D(200.0, 4500.0),
                                       accel: Raw2D(0.0, 0.0),
                                       velocity: Raw2D(0.0, 0.0),
                                       mass: 2000.0,
                                   },
                                   RawNBody {
                                       position: Raw2D(-1500.0, 750.0),
                                       accel: Raw2D(0.0, 0.0),
                                       velocity: Raw2D(0.0, 0.0),
                                       mass: 2000.0,
                                   }];

pub fn raw_nbody() {
    let mut bodies = RawNBodies.to_vec();
    for _ in 0..10000 {
        //calculate accelerations
        for a in 0..bodies.len() {
            bodies[a].accel = Raw2D(0.0, 0.0);
            for b in 0..bodies.len() {
                if a == b {
                    continue;
                }
                let La = bodies[a].position;
                let Lb = bodies[b].position;
                let Ma = bodies[a].mass;
                let Mb = bodies[b].mass;
                let (Dx, Dy, dist) = La.dist(&Lb);
                let force = G_RAW * (Ma * Mb) / (dist * dist);
                let Fx = force * (Dx / Dy);
                let Fy = force * (Dy / Dx);
                let Ax = Fx / Ma;
                let Ay = Fy / Ma;
                bodies[a].accel = Raw2D(bodies[a].accel.0 + Ax, bodies[a].accel.1 + Ay);
            }
        }
        for a in 0..bodies.len() {
            //integrate acceleration into velocity
            let Raw2D(Vx, Vy) = bodies[a].velocity;
            let Raw2D(Ax, Ay) = bodies[a].accel;
            bodies[a].velocity = Raw2D(Vx + Ax * 0.1, Vy + Ay * 0.1);
            //integrate velocity into position
            let Raw2D(Vx, Vy) = bodies[a].velocity;
            let Raw2D(x, y) = bodies[a].position;
            bodies[a].position = Raw2D(x + Vx * 0.1, y + Vy * 0.1);
        }
    }
}
