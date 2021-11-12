import math
import matplotlib.pyplot as plt

beta = 0.001
N = 500
gamma = 0.1
t_max = 100
dt = 0.1
steps = int(t_max / dt)
x = [i * dt for i in range(steps)]
u0 = 1
TOL = 1e-6
mi_max = 20
alpha = beta * N - gamma

def draw_plot(method, filename, x, u, z):
	plt.figure()
	plt.plot(x, u, label='$u(t)$')
	plt.plot(x, z, label='$z(t) = N - u(t)$')
	plt.legend()
	plt.grid()
	plt.xlabel('$t$')
	plt.ylabel('$u(t)$ / $z(t)$')
	plt.title(method)
	plt.savefig(filename + '.png')

#############################################
#				METODA TRAPEZÓW				#
#############################################

u = [u0]
for n in range(steps - 1):
	u_prev = u[n]
	for _ in range(mi_max):
		u_new = u[n] + dt / 2 * ((alpha * u[n] - beta * u[n] ** 2) + (alpha * u_prev - beta * u_prev ** 2))
		if abs(u_new - u_prev) < TOL:
			break
		u_prev = u_new
	u.append(u_new)

z = [N - u[i] for i in range(steps)]

draw_plot('Metoda Picarda', 'Picard', x, u, z)

#############################################

u = [u0]
for n in range(steps - 1):
	mi = 1
	u_prev = u[n]
	for _ in range(mi_max):
		u_new = u_prev - (u_prev - u[n] - dt / 2 * ((alpha * u[n] - beta * u[n] ** 2) + (alpha * u_prev - beta * u_prev ** 2))) / (1 - dt / 2 * (alpha - 2 * beta * u_prev))
		if abs(u_new - u_prev) < TOL:
			break
		u_prev = u_new
	u.append(u_new)

z = [N - u[i] for i in range(steps)]

draw_plot('Iteracja Newtona', 'Newton', x, u, z)

#############################################
#			NIEJAWNA METODA RK2				#
#############################################

a = [[0, 0, 0],
	[0, 0.25, 0.25 - math.sqrt(3) / 6],	
	[0, 0.25 + math.sqrt(3) / 6, 0.25]]
b = [0, 0.5, 0.5]

def f(u):
	return (beta * N - gamma) * u - beta * u ** 2

def F1(u, U1, U2):
	return U1 - u[n] - dt * (a[1][1] * (alpha * U1 - beta * U2 ** 2) + a[1][2] * (alpha * U2 - beta * U2 ** 2))

def F2(u, U1, U2):
	return U2 - u[n] - dt * (a[2][1] * (alpha * U1 - beta * U2 ** 2) + a[2][2] * (alpha * U2 - beta * U2 ** 2))

u = [u0]
for n in range(steps - 1):
	U1 = u[n]
	U2 = u[n]
	mi = 1
	for _ in range(mi_max):
		m = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
		m[1][1] = 1 - dt * a[1][1] * (alpha - 2 * beta * U1)
		m[1][2] = -dt * a[1][2] * (alpha - 2 * beta * U2)
		m[2][1] = -dt * a[2][1] * (alpha - 2 * beta * U1)
		m[2][2] = 1 - dt * a[2][2] * (alpha - 2 * beta * U2)

		U1_new = U1 + (F2(u, U1, U2) * m[1][2] - F1(u, U1, U2) * m[2][2]) / (m[1][1] * m[2][2] - m[1][2] * m[2][1])
		U2_new = U2 + (F1(u, U1, U2) * m[2][1] - F2(u, U1, U2) * m[1][1]) / (m[1][1] * m[2][2] - m[1][2] * m[2][1])

		if (abs(U1_new - U1) < TOL and abs(U2_new - U2) < TOL):
			break
		U1 = U1_new
		U2 = U2_new
	u.append(u[n] + dt * (b[1] * f(U1) + b[2] * f(U2)))

z = [N - u[i] for i in range(steps)]

draw_plot('Niejawna Metoda RK2', 'Niejawna RK2', x, u, z)