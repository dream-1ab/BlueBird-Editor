count = 0

for a in range(20_000):
    for b in range(20_000):
        if (a + b) % 2 == 0:
            count += 1

print(count)