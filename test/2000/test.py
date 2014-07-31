#!/usr/bin/python3

import sys
import time
import subprocess

subprocess.call(["./test/2000/add.sh"])

start = time.time()
subprocess.call(["./test/2000/search_bg.sh"])
elapsed = time.time() - start

print('Search time (4x5 elements) in ' + str(elapsed) + 's')
