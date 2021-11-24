import numpy as np
import matplotlib.pyplot as plt

# V
for k in [16, 8, 4, 2, 1]:
	print('Plotting V ' + str(k))

	plt.figure()

	v = np.loadtxt('v_' + str(k) + '.txt').reshape((128//k+1, 128//k+1))
	x = np.loadtxt('v_grid_x_' + str(k) + '.txt')
	y = np.loadtxt('v_grid_y_' + str(k) + '.txt')

	plt.pcolor(x, y, np.transpose(v), vmin=np.amin(v), vmax=np.amax(v), shading='auto')
	plt.colorbar()

	plt.title('Potencjał $V(x,y)$ ($k = ' + str(k) + '$)')

	plt.savefig('v_' + str(k) + '.png')

# S
print('Plotting S')

plt.figure()

s16 = np.loadtxt('s_16.txt')
s8 = np.loadtxt('s_8.txt')
s4 = np.loadtxt('s_4.txt')
s2 = np.loadtxt('s_2.txt')
s1 = np.loadtxt('s_1.txt')

plt.plot(s16[:, 0], s16[:, 1], label='$k = 16$')
plt.plot(s8[:, 0], s8[:, 1], label='$k = 8$')
plt.plot(s4[:, 0], s4[:, 1], label='$k = 4$')
plt.plot(s2[:, 0], s2[:, 1], label='$k = 2$')
plt.plot(s1[:, 0], s1[:, 1], label='$k = 1$')

plt.xscale('log')
plt.legend()
plt.grid()
plt.xlabel('it')
plt.ylabel('S(it)')
plt.title('Zmiana całki $S^{(k)}(it)$')

plt.savefig('s.png')