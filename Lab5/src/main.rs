use std::f64::consts::PI;
use std::fs::{self, File};
use std::io::{BufWriter, Write};

const DELTA: f64 = 0.2;
const NX: usize = 128;
const NY: usize = 128;
const X_MAX: f64 = DELTA * NX as f64;
const Y_MAX: f64 = DELTA * NY as f64;
const TOL: f64 = 1e-8;

fn calc_vb1(j: usize) -> f64 {
    (PI * DELTA * j as f64 / Y_MAX).sin()
}

fn calc_vb2(i: usize) -> f64 {
    -(2.0 * PI * DELTA * i as f64 / X_MAX).sin()
}

fn calc_vb3(j: usize) -> f64 {
    (PI * DELTA * j as f64 / Y_MAX).sin()
}

fn calc_vb4(i: usize) -> f64 {
    (2.0 * PI * DELTA * i as f64 / X_MAX).sin()
}

fn main() {
    fs::create_dir_all("plots").expect("Unable to create folder!");

    let ks = [16, 8, 4, 2, 1];

    let mut v: [[f64; NY + 1]; NX + 1] = [[0.0; NY + 1]; NX + 1];

    for i in 0..=NX {
        v[i][NY] = calc_vb2(i);
        v[i][0] = calc_vb4(i);
    }

    for j in 0..=NY {
        v[0][j] = calc_vb1(j);
        v[NX][j] = calc_vb3(j);
    }

    for k in ks {
        let mut s_prev;
        let mut s_next = 0.0;
        let mut it = 0;

        let f_s = File::create(format!("plots/s_{}.txt", k)).expect("Unable to create file!");
        let mut f_s = BufWriter::new(f_s);
        let f_v = File::create(format!("plots/v_{}.txt", k)).expect("Unable to create file!");
        let mut f_v = BufWriter::new(f_v);

        println!("Calculating for k = {}", k);
        loop {
            it += 1;

            for i in (k..NX).step_by(k) {
                for j in (k..NY).step_by(k) {
                    v[i][j] = 0.25 * (v[i + k][j] + v[i - k][j] + v[i][j + k] + v[i][j - k]);
                }
            }

            s_prev = s_next;
            s_next = 0.0;
            for i in (0..NX).step_by(k) {
                for j in (0..NY).step_by(k) {
                    s_next += 0.5
                        * k as f64
                        * k as f64
                        * DELTA
                        * DELTA
                        * ((v[i + k][j] - v[i][j] + v[i + k][j + k] - v[i][j + k])
                            / (2.0 * k as f64 * DELTA)
                            * (v[i + k][j] - v[i][j] + v[i + k][j + k] - v[i][j + k])
                            / (2.0 * k as f64 * DELTA)
                            + (v[i][j + k] - v[i][j] + v[i + k][j + k] - v[i + k][j])
                                / (2.0 * k as f64 * DELTA)
                                * (v[i][j + k] - v[i][j] + v[i + k][j + k] - v[i + k][j])
                                / (2.0 * k as f64 * DELTA))
                }
            }

            writeln!(f_s, "{} {}", it, s_next).expect("Unable to write!");
            print!("\r[{}] {}", it, s_next);

            if ((s_next - s_prev) / s_prev).abs() < TOL {
                break;
            }
        }
        println!();

        for i in 1..NX {
            for j in 1..NY {
                writeln!(f_v, "{} {} {}", i as f64 * DELTA, j as f64 * DELTA, v[i][j])
                    .expect("Unable to write!");
            }
        }
    }
}
