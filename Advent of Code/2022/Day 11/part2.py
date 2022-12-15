with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

monkeys = []
old = 0
new = 0
gtest = 1
class Monkey:
    def __init__(self, items, op, test, a, b):
        global gtest
        self.items = eval("[" + items + "]")
        self.op = op
        self.test = int(test)
        gtest *= self.test
        self.a = int(a)
        self.b = int(b)
        self.inspect_count = 0
    
    def inspect(self):
        global old, new
        while len(self.items) != 0:
            self.inspect_count += 1
            old = self.items.pop()
            new = 0
            exec(self.op, globals())
            # new //= 3
            if new % self.test == 0:
                monkeys[self.a].items.append(new % gtest)
            else:
                monkeys[self.b].items.append(new % gtest)
        


for i in range(0, len(data), 7):
    items = data[i+1].split("Starting items: ")[-1]
    op = data[i+2].split("Operation: ")[-1]
    test = data[i+3].split(" ")[-1]
    a = int(data[i+4].split(" ")[-1])
    b = int(data[i+5].split(" ")[-1])
    monkeys.append(Monkey(items, op, test, a, b))

for i in range(10000):
    for monkey in monkeys:
        monkey.inspect()

counts = [monkey.inspect_count for monkey in monkeys]
print(counts)
counts.sort()
print(counts[-1] * counts[-2])