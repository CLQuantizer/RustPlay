class Dog:
    id: str
    def __init__(self, id: int):
        self.id=id

l = [Dog(id=1),Dog(id=2),Dog(id=3)]
dog3 = l[-1]
l.pop()
print(dog3.id)