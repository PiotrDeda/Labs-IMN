import numpy as np
import matplotlib.pyplot as plt

# S global
print('Plotting S global')

plt.figure()

gs06 = np.loadtxt('gs_0.6.txt')
gs1 = np.loadtxt('gs_1.txt')

plt.plot(gs06[:, 0], gs06[:, 1], label='$omega_G = 0.6$')
plt.plot(gs1[:, 0], gs1[:, 1], label='$omega_G = 1.0$')

plt.xscale('log')
plt.legend()
plt.grid()
plt.xlabel('it')
plt.ylabel('S(it)')
plt.title('Zmiana całki S (relaksacja globalna)')

plt.savefig("gs.png")

# S local
print('Plotting S local')

plt.figure()

lsl = np.loadtxt('ls_1.txt')
ls14 = np.loadtxt('ls_1.4.txt')
lsl8 = np.loadtxt('ls_1.8.txt')
lsl9 = np.loadtxt('ls_1.9.txt')

plt.plot(lsl[:, 0], lsl[:, 1], label='$omega_L = 1.0$')
plt.plot(ls14[:, 0], ls14[:, 1], label='$omega_L = 1.4$')
plt.plot(lsl8[:, 0], lsl8[:, 1], label='$omega_L = 1.8$')
plt.plot(lsl9[:, 0], lsl9[:, 1], label='$omega_L = 1.9$')

plt.xscale('log')
plt.legend()
plt.grid()
plt.xlabel('it')
plt.ylabel('S(it)')
plt.title('Zmiana całki $S$ (relaksacja lokalna)')

plt.savefig("ls.png")

# Vn 0.6
print('Plotting Vn 0.6')

plt.figure()

vn06 = np.loadtxt('vn_0.6.txt')

plt.tricontourf(vn06[:, 0], vn06[:, 1], vn06[:, 2], levels=np.linspace(0, 10, 999))
c = plt.colorbar(ticks=np.linspace(0, 10, 11))

plt.title('Zrelaksowany potencjał $V(x,y)$ ($omega_G = 0.6$)')

plt.savefig("vn_0.6.png")

# Vn 1
print('Plotting Vn 1')

plt.figure()

vn1 = np.loadtxt('vn_1.txt')

plt.tricontourf(vn1[:, 0], vn1[:, 1], vn1[:, 2], levels=np.linspace(0, 10, 999))
c = plt.colorbar(ticks=np.linspace(0, 10, 11))

plt.title('Zrelaksowany potencjał $V(x,y)$ ($omega_G = 1.0$)')

plt.savefig("vn_1.png")

# Err 0.6
print('Plotting Err 0.6')

plt.figure()

err06 = np.loadtxt('err_0.6.txt')

plt.tricontourf(err06[:, 0], err06[:, 1], err06[:, 2], levels=np.linspace(0, 0.00275, 999))
c = plt.colorbar(ticks=np.linspace(0, 0.00275, 11))

plt.title('Błąd relaksacji ($omega_G = 0.6$)')

plt.savefig("err_0.6.png")

# Err 1
print('Plotting Err 1')

plt.figure()

err1 = np.loadtxt('err_1.txt')

plt.tricontourf(err1[:, 0], err1[:, 1], err1[:, 2], levels=np.linspace(0, 0.00275, 999))
c = plt.colorbar(ticks=np.linspace(0, 0.00275, 11))

plt.title('Błąd relaksacji ($omega_G = 1.0$)')

plt.savefig("err_1.png")