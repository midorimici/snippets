from random import randint as ri

N = ri(1, 100)
print(N)

A = [ri(1, 1000) for _ in range(N)]
print(A)

'''
for _ in range(N):
  A = ri(1, 1000)
  B = ri(1, 100)
  print(A, B)
'''
