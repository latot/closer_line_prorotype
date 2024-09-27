from sympy import Rational, oo
import matplotlib.pyplot as plt
import numpy

use_floats = False
if use_floats:
  def Rational(x):
    return float(x)

A = [
  [Rational("18.00678831099686"), Rational("69.0404811833497")],
  [Rational("18.00678463099686"), Rational("69.0404543133497")],
  [Rational("18.00677727099686"), Rational("69.0404005833497")]
]

B = [
  [Rational("18.00677727099686"), Rational("69.0404005833497")],
  [Rational("18.006780950996863"), Rational("69.04042744334969")],
  [Rational("18.00678831099686"), Rational("69.0404811833497")],  
]

PS = [Rational("18.006691126034692"), Rational("69.04048768506776")]

def distance(P1, P2):
  return ((P1[0] - P2[0])**Rational(2) + (P1[1] - P2[1])**Rational(2))**Rational(0.5)

def closes_d(P1, P2, PS):
  #P1 = A[0]
  #P2 = A[1]
  #PS = PS
  vpx = P2[0] - P1[0]
  vpy = P2[1] - P1[1]
  tr = numpy.array(range(1000))/ 1000
  t = (vpx * (PS[0] - P1[0]) + vpy * (PS[1] - P1[1])) / (vpx**2 + vpy**2)
  x = vpx * t + P1[0]
  y = vpy * t + P1[1]
  print("x: {}".format(x.n()))
  print("y: {}".format(y.n()))
  plt.plot([P1[0], P2[0]], [P1[1], P2[1]], "ro", color = "red", alpha = 0.5)
  plt.plot(tr*vpx + P1[0], tr*vpy + P1[1])
  plt.plot(x, y, "o", color = "green", alpha = 0.5)
  plt.show()
  x_valid = [min(P1[0], P2[0]), max(P1[0], P2[0])]
  y_valid = [min(P1[1], P2[1]), max(P1[1], P2[1])]
  if x <= x_valid[1] and x >= x_valid[0] and y <= y_valid[1] and y >= y_valid[0]:
    return distance(PS, [x, y])
  print("On corner")
  d1 = distance(PS, P1)
  d2 = distance(PS, P2)
  return min(d1, d2)

distances = {}

for i in range(len(A) - 1):
  d = closes_d(A[i], A[i + 1], PS)
  print("Distance from segment {} of A: {}".format(i, d.n()))
  distances["A" + str(i)] = d

for i in range(len(B) - 1):
  d = closes_d(B[i], B[i + 1], PS)
  print("Distance from segment {} of B: {}".format(i, d.n()))
  distances["B" + str(i)] = d

df = oo
key = ""

for i in distances.keys():
  if distances[i] < df:
    key = i
    df = distances[i]

print("Min Distance: {}".format(df.n()))
print("Key distance: {}".format(key))
