# stupid sim problem!!! cheers

with open("input", "r") as f:
    data = f.readlines()
data = [d.rstrip() for d in data]


# hard-code the setup because lazy
setup = data[:9]
data = data[10:]

stacks = []

indexes = setup[-1]
setup = setup[:-1]
for i in range(len(indexes)):
    if indexes[i] in '123456789':
        stack = []
        for row in setup:
            if len(row) > i and row[i] != ' ':
                stack.append(row[i])

        stacks.append(stack)
    

for d in data:
    d = d.split(" ")
    move = int(d[1]), int(d[3]) - 1, int(d[5]) - 1

    temp = stacks[move[1]][:move[0]]
    stacks[move[1]] = stacks[move[1]][move[0]:]
    temp = temp[::-1]
    stacks[move[2]] = temp + stacks[move[2]]

print(''.join([s[0] for s in stacks]))

    


