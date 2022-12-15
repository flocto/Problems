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

not_beacon = set()
for sensor in sensors:
    x, y, dist = sensor
    if abs(2000000 - y) > dist:
        continue

    for ax in range(x - dist, x + dist + 1):
        if abs(ax - x) + abs(2000000 - y) <= dist:
            not_beacon.add(ax)

for beacon in known_beacons:
    if beacon[1] == 2000000:
        not_beacon.remove(beacon[0])

print(len(not_beacon))