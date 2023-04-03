import labx
#import matplotlib.pyplot as plt


cd = labx.read_labx("owo.labx")
m = cd.messung(0)
t = m.datenreihe("t").values
u = m.datenreihe("U_A1").values
# plt.plot(t, u, ".")
# plt.show()