import time

e = time.perf_counter()

# a = 0
a = 0

# b = 1
b = 1

# i = 2
i = 2

while i <= 70:
    # c = a + b
    c = a + b

    # a = b
    a = b

    # b = c
    b = c

    # i = i + 1
    i += 1

print(b)
print(str(time.perf_counter() - e))
