with open('input.txt') as f:
    lines = f.read().splitlines()

numbers = [int(line) for line in lines]

for i, n in enumerate(numbers):
    for j, m in enumerate(numbers[i+1:]):
        if n + m == 2020:
            print(f'{m} {n}: {m*n}')