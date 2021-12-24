import matplotlib.pyplot as plt
import csv
import numpy as np


data_initial = list()
with open("initial.csv") as fd:
    reader = csv.reader(fd)
    for row in reader:
        data_initial.append([float(i) for i in row])
data_initial = np.array(data_initial)
print(data_initial)

data_wsp = list()
with open("wsp.csv") as fd:
    reader = csv.reader(fd)
    for row in reader:
        data_wsp.append([float(i) for i in row])
data_wsp = np.array(data_wsp)


fig, ax = plt.subplots()
ax.scatter(data_initial[:, 0], data_initial[:, 1], label="Initial")
ax.scatter(data_wsp[:, 0], data_wsp[:, 1], label="WSP", marker="+")
plt.legend()
plt.show()
