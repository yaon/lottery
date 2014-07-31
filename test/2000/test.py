#!/usr/bin/python3

import sys
import time
import subprocess

start = time.time()
subprocess.call(["./test/2000/search_bg.sh"])
elapsed = time.time() - start

print('Search time (4x50 elements) in ' + str(elapsed) + 's')
