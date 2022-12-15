with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

sensors = []
known_beacons = set()

for line in data:
    sensor, beacon = line.split(": ")
    beacon = beacon.split("at ")[-1].split(", ")
    beacon = (int(beacon[0].split("=")[-1]), int(beacon[1].split("=")[-1]))

    sensor = sensor.split("at ")[-1].split(", ")
    x, y = (int(sensor[0].split("=")[-1]), int(sensor[1].split("=")[-1]))
    dist = abs(beacon[0] - x) + abs(beacon[1] - y)
    sensor = (x, y, dist)

    sensors.append(sensor)
    known_beacons.add(beacon)

from tqdm import tqdm
for y in tqdm(range(0, 4_000_001)):
    x = 0
    while x <= 4_000_000:
        if (x, y) in known_beacons:
            x += 1
            continue
        
        possible = True
        nextx = x + 1
        for sensor in sensors:
            sx, sy, dist = sensor
            if abs(sx - x) + abs(sy - y) <= dist:
                nextx = max(nextx, sx + dist - abs(sy - y))
                possible = False
            

        if possible:
            print(x*4000000 + y)
            exit()
        
        x = nextx

# right after this finished running, i found better optimization XD
# just check extrema because hamiltonian distance means diamond
# then bound by extrema -> way faster skipping LOL