import os
import numpy as np


def countWaysToReach(data, end):
    counts = dict()
    newCounts = {node: (1 if node == end else 0) for node in data}

    while newCounts != counts:
        counts = newCounts
        newCounts = {
            node: (1 if node == end
                   else sum(counts[child] for child in data[node]))
            for node in counts
        }

    return newCounts


def part2_first(data):
    print(countWaysToReach(data, 'fft')['svr']
          * countWaysToReach(data, 'dac')['fft']
          * countWaysToReach(data, 'out')['dac']
          + countWaysToReach(data, 'dac')['svr']
          * countWaysToReach(data, 'fft')['dac']
          * countWaysToReach(data, 'out')['fft'])


def countWays_np(data: dict, start, end):
    enums = {key: idx for (idx, key) in enumerate(data.keys())}
    adj = np.zeros((len(data), len(data)))
    for (k, v) in data.items():
        row = enums[k]
        cols = np.array([enums[i] for i in v])
        if len(cols) > 0:
            adj[row, cols] = 1
    old_vec = None
    new_vec = np.zeros((len(data)))
    new_vec[enums[end]] = 1
    while (old_vec != new_vec).any():
        old_vec = new_vec
        new_vec = adj @ old_vec
        new_vec[enums[end]] = 1
    return new_vec[enums[start]]


def part2_second(data: dict):
    print(
        countWays_np(data, 'svr', 'dac') * countWays_np(data, 'dac', 'fft') * countWays_np(data, 'fft', 'out') +
        countWays_np(data, 'svr', 'fft') * countWays_np(data,
                                                        'fft', 'dac') * countWays_np(data, 'dac', 'out')
    )


if __name__ == '__main__':
    script_path = os.path.dirname(os.path.realpath(__file__))
    fname = os.path.join(script_path, "../../data/day11.txt")
    data = dict()
    for line in open(fname):
        i, o = line.strip().split(':')
        data[i] = set(o.split())
    data['out'] = []
    part2_first(data)
    part2_second(data)
