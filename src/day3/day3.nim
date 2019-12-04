import strutils
import tables

proc mapWirePath(path: seq[string]): Table[tuple[x: int, y: int], int] =
    var x = 0
    var y = 0
    var steps = 0

    for section in path:
        let (dx, dy) = case section[0]:
        of 'D': ( 0,-1)
        of 'U': ( 0, 1)
        of 'L': (-1, 0)
        of 'R': ( 1, 0)
        else: raise newException(Exception, "Unexpected direction")

        let dist = parseInt(section[1..^1])
        for d in 0..<dist:
            x += dx
            y += dy
            steps += 1
            discard result.hasKeyOrPut((x, y), steps)

let w1_input = stdin.readLine().split(',')
let w2_input = stdin.readLine().split(',')

let w1_path = mapWirePath(w1_input)
let w2_path = mapWirePath(w2_input)

var part1 = high(int)
var part2 = high(int)

for pos1, step1 in w1_path:
    let step2 = w2_path.getOrDefault pos1
    if step2 > 0:
        let dist = abs(pos1.x) + abs(pos1.y)
        part1 = min(part1, dist)
        let stepSum = step1 + step2
        part2 = min(part2, stepSum)

echo "Part 1: ", part1
echo "Part 2: : ", part2