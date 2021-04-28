#!/usr/bin/python

# This is a simple client to read from the streamer and output the reads.
# I use this script for experimentation, so it also creates a graph of 250 reads from each chip read.

import socket
from datetime import datetime

s = socket.socket()

host = "192.168.0.10"
port = 10000

reads = {}
s.connect((host, port))
count = 0
while count < 250:
    data = s.recv(1024).decode("utf-8").strip()
    data_spl = data.split(",", 2)
    epc = data_spl[0]
    timestamp = datetime.fromtimestamp(int(data_spl[1]) / 1000000.0)
    strength = int(data_spl[2])

    tag_num = int(epc[13:15] + epc[16:18], 16)
    print(tag_num, epc, timestamp.isoformat(), strength)
    reads.setdefault(tag_num, []).append(strength)
    count += 1

import matplotlib.pyplot as plt

for tag, strs in reads.items():
    print(tag, strs)
    plt.plot(list(range(len(strs))), strs)
    plt.show()
