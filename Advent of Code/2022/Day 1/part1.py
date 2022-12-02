with open("input") as f:
    data = f.read().splitlines()

maxSoFar = 0
temp = 0

for line in data:
    if line == "":
        if temp > maxSoFar:
            maxSoFar = temp
        temp = 0
    else:
        temp += int(line)

print(maxSoFar)