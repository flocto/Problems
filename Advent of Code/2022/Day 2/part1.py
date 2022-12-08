with open("input", "r") as f:
    data = f.read().splitlines()

# rock paper scissors
# A is rock, B is paper, C is scissors

score = {"X": 1, "Y": 2, "Z": 3}
played = {"X": "A", "Y": "B", "Z": "C"}
beats = {"A": "C", "B": "A", "C": "B"}

def play(a, b):
    b_real = b
    b = played[b]
    if a == b:
        return 3 + score[b_real]

    if beats[b] == a: 
        return 6 + score[b_real]
    
    return score[b_real]

s = 0
for line in data:
    a, b = line.split()
    s += play(a, b)

print(s)


