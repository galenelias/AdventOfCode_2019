import strutils
import sequtils
import math

# Runs the Intcode emulator, returning the sequence of all output values
proc runSimulation(dataIn: seq[int], inputValue: int): seq[int] =
    var data = dataIn
    var ip = 0

    while data[ip] != 99:
        let opcode = data[ip] mod 100;
        let addressingModes: int = data[ip] div 100

        let getParam = proc (param: int): int=
            let isImmediate = ((addressingModes div (10 ^ (param - 1))) mod 10) == 1
            if isImmediate:
                data[ip + param]
            else:
                data[data[ip + param]]

        case opcode:
        of 1: # ADD
            data[data[ip + 3]] = getParam(1) + getParam(2)
            ip += 4

        of 2: # MUL
            data[data[ip + 3]] = getParam(1) * getParam(2)
            ip += 4

        of 3: # INPUT
            data[data[ip + 1]] = inputValue
            ip += 2

        of 4: # OUTPUT
            result.add(getParam(1))
            ip += 2

        of 5: # JNZ
            if getParam(1) != 0:
                ip = getParam(2)
            else:
                ip += 3

        of 6: # JEZ
            if getParam(1) == 0:
                ip = getParam(2)
            else:
                ip += 3
                
        of 7: # COMPL
            data[data[ip+3]] = if getParam(1) < getParam(2): 1 else: 0
            ip += 4
            
        of 8: # COMPE
            data[data[ip+3]] = if getParam(1) == getParam(2): 1 else: 0
            ip += 4

        else:
            echo "Unhandled opcode: ", opcode
            raise newException(Exception, "Unexpected opcode")

proc solve*(inputs_str: seq[string]): void=
    let data = inputs_str[0].split(',').map(parseInt)

    let part1 = runSimulation(data,  1)
    echo "Part 1: ", part1[part1.len-1]

    let part2 = runSimulation(data,  5)
    echo "Part 2: ", part2[0]

