import os
import sys

start = sys.argv[1]
end = sys.argv[2]

for i in range(int(start), int(end)):
    print("Starting worker: ", i)
    os.system("pm2 start ./target/release/collectionscraper --time --name worker"+str(i))