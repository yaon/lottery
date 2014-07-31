#!/usr/bin/python3

import sys
import time
from subprocess import Popen

start = time.time()
r = Popen(["./search_bg.sh"])
elapsed = time.time() - start

print('Search time (4x50 elements) in ' + str(elapsed) + 's')
