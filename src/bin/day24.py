from z3 import *

vars = []
for i in range(14):
    var = Int(f'w{i}')
    globals()[f'w{i}'] = var
    vars.append(var)

def build_optimizer():
    # after staring at the program and manually decompiling it, i produced this
    # vim substitution:
    #
    # s/inp w\nmul x 0\nadd x z\nmod x 26\ndiv z \([-[:digit:]]*\)\nadd x \([-[:digit:]]*\)\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y \([-[:digit:]]*\)\nmul y x\nadd z y/(25*((z%26 + \2) != w0) + 1) * (z\/\1) + (w0 + \3)*((z%26 + \2) != w0)/
    #
    # run that on the input file to produce the following then reduce a bit more
    # by hand and set the z each time.

    s = Optimize()
    for var in vars:
        s.add(var >= 1)
        s.add(var <= 9)

    z = IntVal(0)
    z = (25+1)*(z/1) + (w0+12)
    z = (25 + 1) * (z/1) + (w1 + 7)
    z = (25 + 1) * (z/1) + (w2 + 8)
    z = (25 + 1) * (z/1) + (w3 + 8)
    z = (25 + 1) * (z/1) + (w4 + 15)
    z = (25*((z%26 + -16) != w5) + 1) * (z/26) + (w5 + 12)*((z%26 + -16) != w5)
    z = (25 + 1) * (z/1) + (w6 + 8)
    z = (25*((z%26 + -11) != w7) + 1) * (z/26) + (w7 + 13)*((z%26 + -11) != w7)
    z = (25*((z%26 + -13) != w8) + 1) * (z/26) + (w8 + 3)*((z%26 + -13) != w8)
    z = (25 + 1) * (z/1) + (w9 + 13)
    z = (25*((z%26 + -8) != w10) + 1) * (z/26) + (w10 + 3)*((z%26 + -8) != w10)
    z = (25*((z%26 + -1) != w11) + 1) * (z/26) + (w11 + 9)*((z%26 + -1) != w11)
    z = (25*((z%26 + -4) != w12) + 1) * (z/26) + (w12 + 4)*((z%26 + -4) != w12)
    z = (25*((z%26 + -14) != w13) + 1) * (z/26) + (w13 + 13)*((z%26 + -14) != w13)

    s.add(z == 0)
    return s

def solve(s):
    s.check()
    m = s.model()
    return ''.join(str(m.eval(var)) for var in vars)

def part1():
    s = build_optimizer()
    for var in vars:
        s.maximize(var)
    print(solve(s))

def part2():
    s = build_optimizer()
    for var in vars:
        s.minimize(var)
    print(solve(s))

if __name__ == '__main__':
    part1()
    part2()
