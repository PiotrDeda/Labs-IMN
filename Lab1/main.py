import math
import matplotlib.pyplot as plt

#############################################
#			PROBLEM AUTONOMICZNY			#
#############################################

l = -1
tmin = 0
tmax = 5
dts = (0.01, 0.1, 1.0)

def solution(t):
	return math.exp(l * t)

def solve(file_name, method_name, recipe):
	plt.figure(0)
	plt.figure(1)

	for dt in dts:
		x = [0]
		y = [1]
		y_solution = [1]
		diff = [0]
		for n in range(int((tmax - tmin) / dt)):
			x.append(round(x[n] + dt, 2))
			y_solution.append(solution(x[n+1]))
			recipe(y, dt, n)
			diff.append(y[n+1] - y_solution[n+1])
		plt.figure(0)
		plt.plot(x, y, label=r'$\Delta t = ' + str(dt) + '$')
		plt.figure(1)
		plt.plot(x, diff, label=r'$\Delta t = ' + str(dt) + '$')
	
	min_dt = min(dts)
	x = [0]
	y = [1]
	for n in range(int((tmax - tmin) / min_dt)):
		x.append(round(x[n] + min_dt, 2))
		y.append(solution(x[n+1]))
	plt.figure(0)
	plt.plot(x, y, label='Analityczne')

	plt.legend()
	plt.grid()
	plt.xlabel('$t$')
	plt.ylabel('$y(t)$')
	plt.title('Metoda jawna ' + method_name)
	plt.savefig(file_name + '.png')

	plt.figure(1)
	plt.legend()
	plt.grid()
	plt.xlabel('$t$')
	plt.ylabel(r'$y_{numeryczne}(t) - y_{analityczne}(t)$')
	plt.title('Różnica metody jawnej ' + method_name +' z rozwiązaniem analitycznym')
	plt.savefig(file_name + ' Diff.png')

	plt.close('all')

#############################################

def euler(y, dt, n):
	y.append(y[n] + dt * l * y[n])

def RK2(y, dt, n):
	k1 = l * y[n]
	k2 = l * (y[n] + dt * k1)
	y.append(y[n] + dt / 2 * (k1 + k2))

def RK4(y, dt, n):
	k1 = l * y[n]
	k2 = l * (y[n] + dt / 2 * k1)
	k3 = l * (y[n] + dt / 2 * k2)
	k4 = l * (y[n] + dt * k3)
	y.append(y[n] + dt / 6 * (k1 + 2 * k2 + 2 * k3 + k4))

solve('Euler', 'Eulera', euler)
solve('RK2', 'RK2 (trapezów)', RK2)
solve('RK4', 'RK4', RK4)

#############################################
#				RRZ 2. RZĘDU				#
#############################################

dt = 10e-4
R = 100
L = 0.1
C = 0.001
om0 = 1 / math.sqrt(L * C)
T0 = 2 * math.pi / om0
tmin = 0
tmax = 4 * T0
omv_mods = (0.5, 0.8, 1.0, 1.2)

plt.figure(0)
plt.figure(1)

for omv_mod in omv_mods:
	x = [0]
	Q = [0]
	I = [0]
	V = lambda n: 10 * math.sin(omv_mod * om0 * (x[0] + n * dt))
	for n in range(int((tmax - tmin) / dt)):
		x.append(x[n] + dt)
		kQ1 = I[n]
		kI1 = V(n) / L - Q[n] / (L * C) - R * I[n] / L
		kQ2 = I[n] + dt / 2 * kI1
		kI2 = V(n + 0.5) / L - (Q[n] + dt / 2 * kQ1) / (L * C) - R * (I[n] + dt / 2 * kI1) / L
		kQ3 = I[n] + dt / 2 * kI2
		kI3 = V(n + 0.5) / L - (Q[n] + dt / 2 * kQ2) / (L * C) - R * (I[n] + dt / 2 * kI2) / L
		kQ4 = I[n] + dt * kI3
		kI4 = V(n + 1) / L - (Q[n] + dt * kQ3) / (L * C) - R * (I[n] + dt * kI3) / L
		Q.append(Q[n] + dt / 6 * (kQ1 + 2 * kQ2 + 2 * kQ3 + kQ4))
		I.append(I[n] + dt / 6 * (kI1 + 2 * kI2 + 2 * kI3 + kI4))
	plt.figure(0)
	plt.plot(x, Q, label=r'$\omega_V = ' + str(omv_mod) + r'\omega_0$')
	plt.figure(1)
	plt.plot(x, I, label=r'$\omega_V = ' + str(omv_mod) + r'\omega_0$')

plt.figure(0)
plt.legend()
plt.grid()
plt.xlabel('$t$')
plt.ylabel('$Q(t)$')
plt.title('RRZ 2. rzędu (metoda jawna RK4) – ładunek')
plt.savefig('RRZ2 Q.png')

plt.figure(1)
plt.legend()
plt.grid()
plt.xlabel('$t$')
plt.ylabel('$I(t)$')
plt.title('RRZ 2. rzędu (metoda jawna RK4) – natężenie')
plt.savefig('RRZ2 I.png')

plt.close('all')