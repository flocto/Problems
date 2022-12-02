with open("input") as f:
    data = f.read().splitlines()

elves = []
temp = 0
for line in data:
    if line == "":
        elves.append(temp)
        temp = 0
    else:
        temp += int(line)

elves.append(temp)

elves.sort()
print(elves[-3] + elves[-2] + elves[-1])