#!/usr/bin/python3

import sys
import time
from subprocess import Popen

if len(sys.argv) != 2:
    print("./test.py nthreads")
    exit(2)

procs = sys.argv[1]

start = time.time()
Popen([sys.executable, "./main", "procs"]).communicate()
Popen([sys.executable, "./client","test/2000/1.loto"]).communicate()
Popen([sys.executable, "./client", "test/2000/2.loto"]).communicate()
Popen([sys.executable, "./client", "test/2000/3.loto"]).communicate()
Popen([sys.executable, "./client", "test/2000/4.loto"]).communicate()
Popen([sys.executable, "./client", "test/2000/5.loto"]).communicate()
elapsed_add = time.time() - start

start = time.time()
a=Popen([sys.executable, "./client", "test/2000/1.loto"])
b=Popen([sys.executable, "./client", "test/2000/2.loto"])
c=Popen([sys.executable, "./client", "test/2000/3.loto"])
d=Popen([sys.executable, "./client", "test/2000/4.loto"])
a.communicate()
b.communicate()
c.communicate()
a.communicate()
elapsed_search = time.time() - start

print('Add time (2000 elements): ' + str(elapsed_add) + 's' + '\n'
      'Search time (4x50 elements) with ' + procs + ' threads in ' + str(elapsed_search) + 's')
