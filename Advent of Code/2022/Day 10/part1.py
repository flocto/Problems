with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

X = 1
signals = []
cycle = 1

marker = 20

for line in data:
    if line != "noop":
        dx = int(line.split(" ")[1])
        if cycle + 2 > marker:
            # print(cycle, marker, line, X)
            signals.append(X * (cycle + 1))
            marker += 40
        cycle += 2
        X += dx
    else:
        cycle += 1

    if cycle == marker:
        # print(cycle, marker)
        signals.append(X * cycle)
        marker += 40
        
print(signals)
print(sum(signals))
