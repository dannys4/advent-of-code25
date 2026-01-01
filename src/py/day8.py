import os
from itertools import combinations
import numpy as np


def process(points: np.ndarray[np.int64], max_connections: int | None):
    points = np.c_[points, -np.ones(len(points), dtype=points.dtype)]
    distances = np.array(
        [(i, j, np.linalg.norm(points[i] - points[j])) for i, j in combinations(range(len(points)), 2)], dtype=points.dtype
    )
    distances = distances[np.argsort(distances[:, 2])]
    if max_connections:
        distances = distances[:max_connections]

    next_circuit = 0
    loop_iter = 0
    for i, j, dist in distances:
        circuit_1 = points[i, 3]
        circuit_2 = points[j, 3]
        if circuit_1 == circuit_2 == -1:
            points[i, 3] = points[j, 3] = next_circuit
            next_circuit += 1
        elif circuit_2 == -1:
            points[j, 3] = circuit_1
        elif circuit_1 == -1:
            points[i, 3] = circuit_2
        elif circuit_1 != circuit_2:
            points[points[:, 3] == circuit_2, 3] = circuit_1
        loop_iter += 1
        if max_connections is None and np.unique(points[:, 3]).size == 1:
            print("({}, {}), {}".format(i, j, loop_iter))
            print("({}, {}), {}".format(
                points[i, :3], points[j, :3], loop_iter))
            return points[i, 0] * points[j, 0]
    ret = np.sort(np.unique_counts(
        points[:, 3][points[:, 3] >= 0]).counts)
    return np.prod(ret[-3:])


if __name__ == '__main__':
    script_path = os.path.dirname(os.path.realpath(__file__))
    fname = os.path.join(script_path, "../../data/day8.txt")
    input_points = np.genfromtxt(
        fname, dtype=np.int64, comments=None, delimiter=",")
    # print(process(input_points.copy(), 1000))
    print(process(input_points.copy(), None))
