import numpy as np
import matplotlib.pyplot as plt

for var in ['psi', 'zeta', 'u', 'v']:
    for Q in [-1000, -4000, 4000]:
        print('Plotting ' + var + ' ' + str(Q))

        plt.figure()

        psi = np.loadtxt(var + '_' + str(Q) + '.txt').reshape((201, 91))
        x = np.loadtxt('grid_x.txt')
        y = np.loadtxt('grid_y.txt')
        plt.pcolor(x, y, np.transpose(psi), vmin=np.nanmin(psi), vmax=np.nanmax(psi), shading='auto')
        plt.colorbar()

        plt.title('$' + var + '(x,y)$ ($Q = ' + str(Q) + '$)')

        plt.savefig(var + '_' + str(Q) + '.png')