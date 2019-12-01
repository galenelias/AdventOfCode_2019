from strutils import parseInt
import sequtils

var inputs: seq[int]
while not endoffile(stdin): 
    inputs.add(parseInt(stdin.readLine()))

proc fuelOf(x: int): int = int(x / 3) - 2

let part1 = inputs.map(fuelOf).foldl(a + b)

let part2 = map(inputs, proc (input: int): int =
    var extraFuel = fuelOf(fuelOf(input))
    while extraFuel > 0:
        result += extraFuel
        extraFuel = fuelOf(extraFuel)
).foldl(a+b)

echo "Part 1: ", part1
echo "Part 2: ", part1 + part2