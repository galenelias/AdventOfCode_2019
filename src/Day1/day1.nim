from strutils import parseInt
import sequtils

var inputs: seq[int]

while not endoffile(stdin): 
    inputs.add(parseInt(stdin.readLine()))

let part1_fuel = inputs.mapIt(int(it / 3) - 2).foldl(a + b)


var part2 = 0
for input in inputs:
    let part1 = int(input / 3) - 2
    var part2_fuel = int(part1 / 3) - 2
    var part2_additional_fuel = 0

    while part2_fuel > 0:
        # echo part2_fuel
        part2_additional_fuel += part2_fuel
        part2_fuel = int(part2_fuel / 3) - 2
    part2 += part1 + part2_additional_fuel

echo "Part 1: ", part1_fuel
echo "Part 2: ", part2