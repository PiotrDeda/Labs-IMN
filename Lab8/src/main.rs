use indicatif::ProgressBar;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};

const NX: usize = 400;
const NY: usize = 90;
const I1: usize = 200;
const I2: usize = 210;
const J1: usize = 50;
const DELTA: f64 = 0.01;
const SIGMA: f64 = 10.0 * DELTA;
const XA: f64 = 0.45;
const YA: f64 = 0.45;
const IT_MAX: usize = 10_000;

fn x(i: usize) -> f64 {
    DELTA * i as f64
}

fn y(j: usize) -> f64 {
    DELTA * j as f64
}

fn create_grids() {
    let f_grid_x =
        File::create(format!("plots/grid_x.txt")).expect("Unable to create file plots/grid_x.txt!");
    let mut f_grid_x = BufWriter::new(f_grid_x);
    let f_grid_y =
        File::create(format!("plots/grid_y.txt")).expect("Unable to create file plots/grid_y.txt!");
    let mut f_grid_y = BufWriter::new(f_grid_y);
    
    for i in 0..=NX {
        writeln!(f_grid_x, "{}", x(i)).expect("Unable to write to plots/grid_y.txt!");
    }

    for j in 0..=NY {
        writeln!(f_grid_y, "{}", y(j)).expect("Unable to write to plots/grid_y.txt!");
    }
}

fn u(x: f64, y: f64) -> f64 {
    (-((x - XA) * (x - XA) + (y - YA) * (y - YA)) / (2.0 * SIGMA * SIGMA)).exp()
        / (2.0 * std::f64::consts::PI * SIGMA * SIGMA)
}

fn main() {
    fs::create_dir_all("plots").expect("Unable to create folder plots!");

    create_grids();

    let file = BufReader::new(File::open("psi.dat").expect("Can't open file psi.dat!"));
    let mut psi = vec![vec![0.0; NY+1]; NX+1];
    for line in file.lines() {
        let lt = line.unwrap();
		let mut l = lt.split_whitespace();
		let i: usize = l.next().unwrap().parse().unwrap();
		let j: usize = l.next().unwrap().parse().unwrap();
		let temp_psi = l.next().unwrap().parse().unwrap();
		psi[i][j] = temp_psi;
    }

    let mut vx = vec![vec![0.0; NY+1]; NX+1];
	let mut vy = vec![vec![0.0; NY+1]; NX+1];

    // Vx i Vy
    for i in 1..NX {
        for j in 1..NY {
            vx[i][j] = (psi[i][j + 1] - psi[i][j - 1]) / (2.0 * DELTA);
            vy[i][j] = -(psi[i + 1][j] - psi[i - 1][j]) / (2.0 * DELTA);
        }
    }

    // Zastawka
    for i in I1..=I2 {
        for j in 0..=J1 {
            vx[i][j] = 0.0;
            vy[i][j] = 0.0;
        }
    }

    // Dolny i górny brzeg
    for i in 1..NX {
        vx[i][0] = 0.0;
        vy[i][NY] = 0.0;
    }

    // Lewy i prawy brzeg
    for j in 0..=NY {
        vx[0][j] = vx[1][j];
        vx[NX][j] = vx[NX - 1][j];
    }

	let f_vx = File::create("plots/vx.txt")
		.expect("Unable to create file plots/vx.txt!");
	let mut f_vx = BufWriter::new(f_vx);
	let f_vy = File::create("plots/vy.txt")
		.expect("Unable to create file plots/vy.txt!");
	let mut f_vy = BufWriter::new(f_vy);

	for i in 0..=NX {
		for j in 0..=NY {
			write!(f_vx, "{} ", vx[i][j]).expect("Unable to write to plots/vx.txt!");
			write!(f_vy, "{} ", vy[i][j]).expect("Unable to write to plots/vy.txt!");
		}
		writeln!(f_vx).expect("Unable to write to plots/vx.txt!");
		writeln!(f_vy).expect("Unable to write to plots/vy.txt!");
	}

    let mut v_max = f64::EPSILON;
    let mut v_magnitute: f64;
    for i in 0..=NX {
        for j in 0..=NY {
            v_magnitute = ((vx[i][j]) * (vx[i][j]) + (vy[i][j]) * (vy[i][j])).sqrt();
            if v_magnitute > v_max {
                v_max = v_magnitute;
            }
        }
    }
    let dt = DELTA / (4.0 * v_max);
    println!("Vmax = {}\ndt = {}", v_max, dt);

	let t = |it: usize| -> f64 {
		dt * it as f64
	};
	
	let f_grid_t =
        File::create(format!("plots/grid_t.txt")).expect("Unable to create file plots/grid_t.txt!");
    let mut f_grid_t = BufWriter::new(f_grid_t);
	
	for it in 0..IT_MAX {
        writeln!(f_grid_t, "{}", t(it)).expect("Unable to write to plots/grid_t.txt!");
    }

    let mut u0: [[f64; NY + 1]; NX + 1] = [[0.0; NY + 1]; NX + 1];
    let mut u1: [[f64; NY + 1]; NX + 1] = [[0.0; NY + 1]; NX + 1];

    // Równanie AD
    for d in [0.0, 0.1] {
        println!("\nCalculating for D = {}", d);
        let bar = ProgressBar::new(IT_MAX.try_into().unwrap());
		
        let f_c = File::create(format!("plots/c_{}.txt", d))
            .expect(&format!("Unable to create file plots/c_{}.txt!", d)[..]);
        let mut f_c = BufWriter::new(f_c);
        let f_x_sr = File::create(format!("plots/x_sr_{}.txt", d))
            .expect(&format!("Unable to create file plots/x_sr_{}.txt!", d)[..]);
        let mut f_x_sr = BufWriter::new(f_x_sr);

        for i in 0..=NX {
            for j in 0..=NY {
                u0[i][j] = u(x(i), y(j));
            }
        }

        for it in 1..=IT_MAX {
            bar.inc(1);
            u1.clone_from(&u0);

            for _k in 1..=20 {
                for i in 0..=NX {
                    for j in 1..NY {
                        match (i, j) {
                            // Zastawka
                            (I1..=I2, 0..=J1) => {
                                continue;
                            }
                            // Brzeg
                            (0, _) => {
                                u1[i][j] = (u0[i][j]
                                    - dt * vx[i][j] / 2.0
                                        * (((u0[i + 1][j] - u0[NX][j]) / (2.0 * DELTA))
                                            + ((u1[i + 1][j] - u1[NX][j]) / (2.0 * DELTA)))
                                    - dt * vy[i][j] / 2.0
                                        * (((u0[i][j + 1] - u0[i][j - 1]) / (2.0 * DELTA))
                                            + ((u1[i][j + 1] - u1[i][j - 1]) / (2.0 * DELTA)))
                                    + dt * d / 2.0
                                        * ((u0[i + 1][j]
                                            + u0[NX][j]
                                            + u0[i][j + 1]
                                            + u0[i][j - 1]
                                            - 4.0 * u0[i][j])
                                            / (DELTA * DELTA)
                                            + ((u1[i + 1][j]
                                                + u1[NX][j]
                                                + u1[i][j + 1]
                                                + u1[i][j - 1])
                                                / (DELTA * DELTA))))
                                    / (1.0 + (2.0 * d * dt) / (DELTA * DELTA));
                            }
							(NX, _) => {
                                u1[i][j] = (u0[i][j]
                                    - dt * vx[i][j] / 2.0
                                        * (((u0[0][j] - u0[i - 1][j]) / (2.0 * DELTA))
                                            + ((u1[0][j] - u1[i - 1][j]) / (2.0 * DELTA)))
                                    - dt * vy[i][j] / 2.0
                                        * (((u0[i][j + 1] - u0[i][j - 1]) / (2.0 * DELTA))
                                            + ((u1[i][j + 1] - u1[i][j - 1]) / (2.0 * DELTA)))
                                    + dt * d / 2.0
                                        * ((u0[0][j]
                                            + u0[i - 1][j]
                                            + u0[i][j + 1]
                                            + u0[i][j - 1]
                                            - 4.0 * u0[i][j])
                                            / (DELTA * DELTA)
                                            + ((u1[0][j]
                                                + u1[i - 1][j]
                                                + u1[i][j + 1]
                                                + u1[i][j - 1])
                                                / (DELTA * DELTA))))
                                    / (1.0 + (2.0 * d * dt) / (DELTA * DELTA));
                            }
                            // Reszta
                            _ => {
                                u1[i][j] = (u0[i][j]
                                    - dt * vx[i][j] / 2.0
                                        * (((u0[i + 1][j] - u0[i - 1][j]) / (2.0 * DELTA))
                                            + ((u1[i + 1][j] - u1[i - 1][j]) / (2.0 * DELTA)))
                                    - dt * vy[i][j] / 2.0
                                        * (((u0[i][j + 1] - u0[i][j - 1]) / (2.0 * DELTA))
                                            + ((u1[i][j + 1] - u1[i][j - 1]) / (2.0 * DELTA)))
                                    + dt * d / 2.0
                                        * ((u0[i + 1][j]
                                            + u0[i - 1][j]
                                            + u0[i][j + 1]
                                            + u0[i][j - 1]
                                            - 4.0 * u0[i][j])
                                            / (DELTA * DELTA)
                                            + ((u1[i + 1][j]
                                                + u1[i - 1][j]
                                                + u1[i][j + 1]
                                                + u1[i][j - 1])
                                                / (DELTA * DELTA))))
                                    / (1.0 + (2.0 * d * dt) / (DELTA * DELTA));
                            }
                        }
                    }
                }
            }
            u0.clone_from(&u1);

            // c
			let mut c_temp = 0.0;
            for i in 0..=NX {
                for j in 0..=NY {
                    c_temp += u0[i][j] * DELTA * DELTA;
                }
            }
			writeln!(f_c, "{}", c_temp).expect(&format!("Unable to write to plots/c_{}.txt!", d)[..]);

            // x_sr
			let mut x_sr_temp = 0.0;
            for i in 0..=NX {
                for j in 0..=NY {
                    x_sr_temp += x(i) * u0[i][j] * DELTA * DELTA;
                }
            }
			writeln!(f_x_sr, "{}", x_sr_temp).expect(&format!("Unable to write to plots/x_sr_{}.txt!", d)[..]);

            for k in 1..=5 {
                if t(it) == k as f64 * t(IT_MAX) / 5.0 {
                    let f_u = File::create(format!("plots/u_{}_{}.txt", d, k))
                        .expect(&format!("Unable to create file plots/u_{}_{}.txt!", d, k)[..]);
                    let mut f_u = BufWriter::new(f_u);
                    for i in 0..=NX {
                        for j in 0..=NY {
                            write!(f_u, "{} ", u0[i][j]).expect(
                                &format!("Unable to write to plots/u_{}_{}.txt!", d, k)[..],
                            );
                        }
                        writeln!(f_u)
                            .expect(&format!("Unable to write to plots/u_{}_{}.txt!", d, k)[..]);
                    }
                }
            }
        }

        bar.finish();
    }
}
