with open("input", "r") as f:
    data = f.read()


for i in range(14, len(data)):
    sub = set(data[i-14:i]) # totally efficient
    if len(sub) == 14:
        print(data[i-14:i], i)
        break
    