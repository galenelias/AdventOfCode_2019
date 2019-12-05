from strutils import parseInt
import sequtils

proc fuelOf(x: int): int = int(x / 3) - 2

proc solve*(inputs_str: seq[string]): void=
    let inputs = inputs_str.map(parseInt)

    let part1 = inputs.map(fuelOf).foldl(a + b)

    let part2 = inputs.map(proc (input: int): int =
        var extraFuel = fuelOf(fuelOf(input))
        while extraFuel > 0:
            result += extraFuel
            extraFuel = fuelOf(extraFuel)
    ).foldl(a+b)

    echo "Part 1: ", part1
    echo "Part 2: ", part1 + part2