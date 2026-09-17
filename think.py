idxs = [-5, -5, 9, 10]

offsets = [18*i for i in range(5)]

res = []
for offset in offsets:
    res.extend([(i + offset)%90 if i >= 0 else 999 for i in idxs])

print(res)
