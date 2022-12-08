with open("input", "r") as f:
    data = f.read()


for i in range(4, len(data)):
    sub = set(data[i-4:i]) # totally efficient
    if len(sub) == 4:
        print(data[i-4:i], i)
        break
    