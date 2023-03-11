import numpy as np
import matplotlib.pyplot as plt

for vs in ['vx', 'vy']:
    print('Plotting ' + vs)

    plt.figure()

    v = np.loadtxt(vs + '.txt').reshape((401, 91))
    x = np.loadtxt('grid_x.txt')
    y = np.loadtxt('grid_y.txt')
    plt.pcolor(x, y, np.transpose(v), vmin=np.nanmin(v), vmax=np.nanmax(v), shading='auto')
    plt.colorbar()

    plt.title('$'+ vs +'(x,y)$')

    plt.savefig(vs + '.png')

print('Plotting c and x_sr')

plt.figure()

x = np.loadtxt('grid_t.txt')
y1 = np.loadtxt('c_0.txt')
y2 = np.loadtxt('c_0.1.txt')
y3 = np.loadtxt('x_sr_0.txt')
y4 = np.loadtxt('x_sr_0.1.txt')

plt.plot(x, y1, label='$c$, $D=0$')
plt.plot(x, y2, label='$c$, $D=0.1$')
plt.plot(x, y3, label='$x_sr$, $D=0$')
plt.plot(x, y4, label='$x_sr$, $D=0.1$')

plt.legend()
plt.title('$c$, $x_sr$')

plt.savefig('c_x_sr.png')

for D in [0, 0.1]:
    for k in [1, 2, 3, 4, 5]:
        print('Plotting u_' + str(D) + '_' + str(k))

        plt.figure()

        u = np.loadtxt('u_' + str(D) + '_' + str(k) + '.txt').reshape((401, 91))
        x = np.loadtxt('grid_x.txt')
        y = np.loadtxt('grid_y.txt')
        plt.pcolor(x, y, np.transpose(u), vmin=np.nanmin(u), vmax=np.nanmax(u), shading='auto')
        plt.colorbar()

        plt.title('$u(x,y)$ ($D = ' + str(D) + '$) ($k = ' + str(k) + '$)')

        plt.savefig('u_' + str(D) + '_' + str(k) + '.png')