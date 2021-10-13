import numpy as np
import matplotlib.pyplot as pyplot

from mpl_toolkits.mplot3d import Axes3D
from sklearn.cluster import KMeans
from sklearn import datasets


np.random.seed(5)

iris = datasets.load_iris()

X = iris.data
Y = iris.target
