import better_ode
import matplotlib.pyplot as plt


def f(t, y):
    return [0.02*y[0]-0.00004*y[0]*y[1], 0.00004*y[0]*y[1]-0.04*y[1]]

x = better_ode.solve_diffeq_system(f, [1500, 2200])

data = x.series_return
series1 = data[0]
series2 = data[1]
time = [x[0] for x in series1]
d1 = [x[1] for x in series1]
d2 = [x[1] for x in series2]

plt.plot(time, d1)
plt.plot(time, d2)
plt.xlabel("Time (days)")
plt.ylabel("Population")
plt.show()
