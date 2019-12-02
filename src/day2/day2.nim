import strutils
import sequtils

proc runSimulation(dataIn: seq[int], noun: int, verb: int): int =
    var data = dataIn
    data[1] = noun
    data[2] = verb

    var ip = 0
    while data[ip] != 99:
        let opcode = data[ip]
        case opcode:
        of 1:
            data[data[ip + 3]] = data[data[ip + 1]] + data[data[ip + 2]]
        of 2:
            data[data[ip + 3]] = data[data[ip + 1]] * data[data[ip + 2]]
        else:
            raise newException(Exception, "Unexpected")
        ip += 4
    return data[0]

let input = stdin.readLine()
let data = input.split(',').map(parseInt)

echo "Part 1: ", runSimulation(data, 12, 2)

for noun in 0..99:
    for verb in 0..99:
        if runSimulation(data, noun, verb) == 19690720:
            echo "Part 2: ", noun * 100 + verb
