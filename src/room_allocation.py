from collections import deque


n = int(input())
clients = []

free_rooms = deque([1])
max_room = 1
ans = list(range(n))

for _ in range(n):
    a, b = map(int, input().split())
    clients.extend([(a, -1, _), (b, 1, _)])

clients.sort()
# print(clients)
for d, act, idx in clients:
    if act == -1:  # enter
        if len(free_rooms) == 0:
            max_room += 1
            free_rooms.appendleft(max_room)
        room = free_rooms.popleft()
        ans[idx] = room
    else:
        room = ans[idx]
        free_rooms.appendleft(room)

print(max_room)
print(" ".join(map(str, ans)))
