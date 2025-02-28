import os
import sys

start = sys.argv[1]
end = sys.argv[2]

for i in range(int(start), int(end)):
    print("Stopping worker: ", i)
    os.system("pm2 stop worker"+str(i))