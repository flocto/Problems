with open("input", "r") as f:
    data = f.read().splitlines()

# rock paper scissors
# A is rock, B is paper, C is scissors

score = {"A": 1, "B": 2, "C": 3}
played = {"X": "A", "Y": "B", "Z": "C"}
beats = {"A": "C", "B": "A", "C": "B"}
loses = {"A": "B", "B": "C", "C": "A"}

def play(a, b):
    if b == 'X':  
        return score[beats[a]]
    if b == 'Y':
        return 3 + score[a]
    if b == 'Z':
        return 6 + score[loses[a]]   

s = 0
for line in data:
    a, b = line.split()
    s += play(a, b)

print(s)


