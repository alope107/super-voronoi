idxs = [87, 4, 5, 14]

offsets = [18*i for i in range(5)]

res = []
for offset in offsets:
    res.extend([(i + offset)%90 for i in idxs])

print(res)