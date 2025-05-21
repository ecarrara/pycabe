def simple_complexity():
    return "simple"


def medium_complexity(num):
    if num > 10:
        return "high"
    elif num > 5:
        return "medium"
    else:
        return "low"


def high_complexity(x, y, z):
    if x > 10:
        if y > 20:
            if z > 30:
                return "all high"
            else:
                return "x,y high"
        elif z > 30:
            return "x,z high"
        else:
            return "x high"
    elif y > 20:
        if z > 30:
            return "y,z high"
        else:
            return "y high"
    elif z > 30:
        return "z high"
    else:
        return "all low"


class A:
    def __init__(self):
        self.value = 0

    def nested_complexity(self, a, b, c):
        if a:
            for i in range(b):
                while c > 0:
                    if i % 2 == 0:
                        self.value += 1
                    else:
                        self.value -= 1
                    c -= 1
        return self.value
