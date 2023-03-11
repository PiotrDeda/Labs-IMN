use std::fs::{self, File};
use std::io::{BufWriter, Write};

const DELTA: f64 = 0.01;
const RO: f64 = 1.0;
const MI: f64 = 1.0;
const NX: usize = 200;
const NY: usize = 90;
const I1: usize = 50;
const J1: usize = 55;
const J2: usize = J1 + 2;
const IT_MAX: usize = 20_000;

fn x(i: usize) -> f64 {
    DELTA * i as f64
}

fn y(j: usize) -> f64 {
    DELTA * j as f64
}

fn q_wy(q_we: f64) -> f64 {
    q_we * (y(NY) * y(NY) * y(NY) - y(J1) * y(J1) * y(J1) - 3.0 * y(J1) * y(NY) * y(NY)
        + 3.0 * y(J1) * y(J1) * y(NY))
        / (y(NY) * y(NY) * y(NY))
}

fn is_edge(i: usize, j: usize) -> bool {
    match (i, j) {
        (0, J1..=NY) => true, // A
        (_, NY) => true,      // B
        (NX, _) => true,      // C
        (I1..=NX, 0) => true, // D
        (I1, 0..=J1) => true, // E
        (0..=I1, J1) => true, // F
        _ => false,
    }
}

fn main() {
	fs::create_dir_all("plots").expect("Unable to create folder!");
	
	let f_grid_x = File::create(format!("plots/grid_x.txt")).expect("Unable to create file!");
    let mut f_grid_x = BufWriter::new(f_grid_x);
	let f_grid_y = File::create(format!("plots/grid_y.txt")).expect("Unable to create file!");
    let mut f_grid_y = BufWriter::new(f_grid_y);
	
	for i in 0..=NX {
		writeln!(f_grid_x, "{}", x(i)).expect("Unable to write!");
	}

	for j in 0..=NY {
		writeln!(f_grid_y, "{}", y(j)).expect("Unable to write!");
	}
	
	for q in [-1000.0, -4000.0, 4000.0] {
		println!("Calculating for Q_we = {}", q);
		
		let f_psi = File::create(format!("plots/psi_{}.txt", q)).expect("Unable to create file!");
		let mut f_psi = BufWriter::new(f_psi);
		let f_zeta = File::create(format!("plots/zeta_{}.txt", q)).expect("Unable to create file!");
		let mut f_zeta = BufWriter::new(f_zeta);
		let f_u = File::create(format!("plots/u_{}.txt", q)).expect("Unable to create file!");
		let mut f_u = BufWriter::new(f_u);
		let f_v = File::create(format!("plots/v_{}.txt", q)).expect("Unable to create file!");
		let mut f_v = BufWriter::new(f_v);
		
		let mut psi: [[f64; NY + 1]; NX + 1] = [[0.0; NY + 1]; NX + 1];
		let mut zeta: [[f64; NY + 1]; NX + 1] = [[0.0; NY + 1]; NX + 1];
		let mut u: [[f64; NY + 1]; NX + 1] = [[0.0; NY + 1]; NX + 1];
		let mut v: [[f64; NY + 1]; NX + 1] = [[0.0; NY + 1]; NX + 1];

		////////////////////////
		// Warunek brzegowy ψ //
		////////////////////////

		// A (wejście)
		for j in J1..=NY {
			psi[0][j] = q / (2.0 * MI)
				* ((y(j) * y(j) * y(j) / 3.0 - y(j) * y(j) * (y(J1) + y(NY)) / 2.0
					+ y(j) * y(J1) * y(NY)));
		}

		// C (wyjście)
		for j in 0..=NY {
			psi[NX][j] = q_wy(q) / (2.0 * MI) * (y(j) * y(j) * y(j) / 3.0 - y(j) * y(j) * y(NY) / 2.0)
				+ (q * y(J1) * y(J1) * (-y(J1) + 3.0 * y(NY))) / (12.0 * MI);
		}

		// B
		for i in 1..NX {
			psi[i][NY] = psi[0][NY];
		}

		// D
		for i in I1..NX {
			psi[i][0] = psi[0][J1];
		}

		// E
		for j in 1..=J1 {
			psi[I1][j] = psi[0][J1];
		}

		// F
		for i in 1..=I1 {
			psi[i][J1] = psi[0][J1];
		}

		////////////////////////////
		// Algorytm relaksacji NS //
		////////////////////////////

		let mut omega;
		let mut zeta_temp;
		let mut gamma;

		for it in 1..=IT_MAX {
			omega = !(it < 2000);

			for i in 1..NX {
				for j in 1..NY {
					if !is_edge(i, j) {
						// ψ
						psi[i][j] = (psi[i + 1][j] + psi[i - 1][j] + psi[i][j + 1] + psi[i][j - 1]
							- DELTA * DELTA * zeta[i][j])
							/ 4.0;

						// ζ
						zeta_temp =
							(zeta[i + 1][j] + zeta[i - 1][j] + zeta[i][j + 1] + zeta[i][j - 1]) / 4.0;
						zeta[i][j] = if !omega {
							zeta_temp
						} else {
							zeta_temp
								- RO / (16.0 * MI)
									* ((psi[i][j + 1] - psi[i][j - 1])
										* (zeta[i + 1][j] - zeta[i - 1][j])
										- (psi[i + 1][j] - psi[i - 1][j])
											* (zeta[i][j + 1] - zeta[i][j - 1]))
						};
						
						// u
						u[i][j] = (psi[i][j+1] - psi[i][j-1])/(2.0*DELTA);
						
						// v
						v[i][j] = -(psi[i+1][j] - psi[i-1][j])/(2.0*DELTA);
					}
				}
			}

			////////////////////////
			// Warunek brzegowy ζ //
			////////////////////////

			// A (wejście)
			for j in J1..=NY {
				zeta[0][j] = q / (2.0 * MI) * (2.0 * y(j) - y(J1) - y(NY));
			}

			// C (wyjście)
			for j in 0..=NY {
				zeta[NX][j] = q_wy(q) / (2.0 * MI) * (2.0 * y(j) - y(NY));
			}

			// B
			for i in 1..NX {
				zeta[i][NY] = 2.0 / (DELTA * DELTA) * (psi[i][NY - 1] - psi[i][NY]);
			}

			// D
			for i in I1 + 1..NX {
				zeta[i][0] = 2.0 / (DELTA * DELTA) * (psi[i][1] - psi[i][0]);
			}

			// E
			for j in 1..J1 {
				zeta[I1][j] = 2.0 / (DELTA * DELTA) * (psi[I1 + 1][j] - psi[I1][j]);
			}

			// F
			for i in 1..=I1 {
				zeta[i][J1] = 2.0 / (DELTA * DELTA) * (psi[i][J1 + 1] - psi[i][J1]);
			}

			// Wierzchołek E/F
			zeta[I1][J1] = (zeta[I1 - 1][J1] + zeta[I1][J1 - 1]) / 2.0;

			//////////////////////
			// Kontrola błędu Γ //
			//////////////////////

			gamma = 0.0;
			for i in 1..NX {
				gamma += psi[i + 1][J2] + psi[i - 1][J2] + psi[i][J2 + 1] + psi[i][J2 - 1]
					- 4.0 * psi[i][J2]
					- DELTA * DELTA * zeta[i][J2];
			}
			print!("\r[{}] Gamma = {}                       ", it, gamma);
		}

		for i in 0..I1 {
			psi[i][0] = f64::NAN;
			zeta[i][0] = f64::NAN;
			u[i][0] = f64::NAN;
			v[i][0] = f64::NAN;
		}
		for j in 0..J1 {
			psi[0][j] = f64::NAN;
			zeta[0][j] = f64::NAN;
			u[0][j] = f64::NAN;
			v[0][j] = f64::NAN;
		}
		
		for i in 0..=NX {
			for j in 0..=NY {
				write!(f_psi, "{} ", psi[i][j]).expect("Unable to write!");
				write!(f_zeta, "{} ", zeta[i][j]).expect("Unable to write!");
				write!(f_u, "{} ", u[i][j]).expect("Unable to write!");
				write!(f_v, "{} ", v[i][j]).expect("Unable to write!");
			}
			writeln!(f_psi).expect("Unable to write!");
			writeln!(f_zeta).expect("Unable to write!");
			writeln!(f_u).expect("Unable to write!");
			writeln!(f_v).expect("Unable to write!");
		}
		println!();
	}
}