import numpy as np
import matplotlib.pyplot as plt

for it in [100, 200, 500, 1000, 2000]:
	print('Plotting t_' + str(it))

	plt.figure()

	t = np.loadtxt('t_' + str(it) + '.txt').reshape((41, 41))
	x = np.arange(0, 41)
	y = np.arange(0, 41)
	plt.pcolor(x, y, np.transpose(t), vmin=np.nanmin(t), vmax=np.nanmax(t), shading='auto')
	plt.colorbar()

	plt.title('$T(x,y)$ ($it = ' + str(it) + '$)')

	plt.savefig('t_' + str(it) + '.png')
	
	print('Plotting dt_' + str(it))

	plt.figure()

	dt = np.loadtxt('dt_' + str(it) + '.txt').reshape((41, 41))
	x = np.arange(0, 41)
	y = np.arange(0, 41)
	plt.pcolor(x, y, np.transpose(dt), vmin=np.nanmin(dt), vmax=np.nanmax(dt), shading='auto')
	plt.colorbar()

	plt.title(r'$\nabla^2T(x,y)$ ($it = ' + str(it) + '$)')

	plt.savefig('dt_' + str(it) + '.png')