import os
from itertools import combinations

import numpy as np
import shapely


def day9(points):
    polygon = shapely.Polygon(points)

    largest_area_p1 = 0
    largest_area_p2 = 0

    for p1, p2 in combinations(points, 2):
        x_min, x_max = min(p1[0], p2[0]), max(p1[0], p2[0])
        y_min, y_max = min(p1[1], p2[1]), max(p1[1], p2[1])

        area = (x_max - x_min + 1) * (y_max - y_min + 1)
        largest_area_p1 = max(largest_area_p1, area)

        if polygon.contains(shapely.box(x_min, y_min, x_max, y_max)):
            largest_area_p2 = max(largest_area_p2, area)

    return largest_area_p2


if __name__ == '__main__':
    script_path = os.path.dirname(os.path.realpath(__file__))
    fname = os.path.join(script_path, "../../data/day9.txt")
    input_points = np.genfromtxt(
        fname, dtype=np.int64, comments=None, delimiter=",")
    # print(process(input_points.copy(), 1000))
    print(day9(input_points.copy()))
