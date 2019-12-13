import deques
import math
import sequtils
import strutils
import os

type IntCodeComputer = object
    data: seq[int]
    ip: int
    relativeOffset: int
    inputQueue: Deque[int]
    outputQueue: Deque[int]

proc initCpu(data: seq[int], inputs: seq[int]): IntCodeComputer=
    var cpu = IntCodeComputer(data: data, inputQueue: initDeque[int](), outputQueue: initDeque[int]())
    for i in inputs:
        cpu.inputQueue.addLast(i)
    return cpu

proc setAddr(s: var IntCodeComputer, address: int, value: int): void=
    if address >= s.data.len:
        s.data.setLen(address+1)
    s.data[address] = value

proc readAddr(s: IntCodeComputer, address: int): int=
    if address >= s.data.len:
        return 0
    else:
        return s.data[address]

# Runs the Intcode emulator until we either halt or block on input. Returns whether we hit a HALT instruction
proc runSimulation(cpu: var IntCodeComputer): bool =
    while cpu.data[cpu.ip] != 99:
        let opcode = cpu.data[cpu.ip] mod 100;
        let addressingModes: int = cpu.data[cpu.ip] div 100

        let getParam = proc (cpu: IntCodeComputer, param: int): int=
            let mode = ((addressingModes div (10 ^ (param - 1))) mod 10)
            if mode == 0: #Position mode
                cpu.readAddr(cpu.readAddr(cpu.ip + param))
            elif mode == 1: #Immediate mode
                cpu.readAddr(cpu.ip + param)
            elif mode == 2: #Relative mode
                cpu.readAddr(cpu.relativeOffset + cpu.readAddr(cpu.ip + param))
            else:
                raise newException(Exception, "Unexpected opcode")

        let getDest = proc (cpu: IntCodeComputer, param: int): int=
            let mode = ((addressingModes div (10 ^ (param - 1))) mod 10)
            if mode == 0: #Position mode
                cpu.readAddr(cpu.ip + param)
            elif mode == 2: #Relative mode
                cpu.relativeOffset + cpu.readAddr(cpu.ip + param)
            else:
                echo "Unexpected ouput addressing mode: ", mode
                raise newException(Exception, "Unexpected opcode")

        case opcode:
        of 1: # ADD
            cpu.setAddr(getDest(cpu, 3), getParam(cpu, 1) + getParam(cpu, 2))
            cpu.ip += 4

        of 2: # MUL
            cpu.setAddr(getDest(cpu, 3), getParam(cpu, 1) * getParam(cpu, 2))
            cpu.ip += 4

        of 3: # INPUT
            # Pause execution if we don't have enough input
            if cpu.inputQueue.len == 0:
                return false

            cpu.setAddr(getDest(cpu, 1), cpu.inputQueue.popFirst)
            cpu.ip += 2

        of 4: # OUTPUT
            cpu.outputQueue.addLast(getParam(cpu, 1))
            cpu.ip += 2

        of 5: # JNZ
            if getParam(cpu, 1) != 0:
                cpu.ip = getParam(cpu, 2)
            else:
                cpu.ip += 3

        of 6: # JEZ
            if getParam(cpu, 1) == 0:
                cpu.ip = getParam(cpu, 2)
            else:
                cpu.ip += 3
                
        of 7: # COMPL
            cpu.setAddr(getDest(cpu, 3), if getParam(cpu, 1) < getParam(cpu, 2): 1 else: 0)
            cpu.ip += 4
            
        of 8: # COMPE
            cpu.setAddr(getDest(cpu, 3), if getParam(cpu, 1) == getParam(cpu, 2): 1 else: 0)
            cpu.ip += 4

        of 9: # ADJUST RELATIVE OFFSET
            cpu.relativeOffset += getParam(cpu, 1)
            cpu.ip += 2

        else:
            echo "Unhandled opcode: ", opcode
            raise newException(Exception, "Unexpected opcode")

    return true

type
    CompassDirections = enum
        cdNorth, cdEast, cdSouth, cdWest

proc part1(data: seq[int]): void=
    var cpu = initCpu(data, @[])
    discard cpu.runSimulation()

    var blocks = 0
    for i in 0..<cpu.outputQueue.len() div 3:
        if cpu.outputQueue[i * 3 + 2] == 2:
            blocks += 1

    echo "Part 1: ", blocks

proc part2(dataIn: seq[int], verbose: bool): void=
    var data = dataIn
    data[0] = 2 # insert 2 quarters
    var cpu = initCpu(data, @[])

    var ballPos = (0, 0)
    var paddlePos = (0, 0)
    var score = 0

    const GridRows = 20
    const GridCols = 44
    type GridArray = array[GridRows, array[GridCols, char]]
    var grid: GridArray

    while true:
        let didHalt = cpu.runSimulation()

        for i in 0..<cpu.outputQueue.len() div 3:
            let pos = (cpu.outputQueue[i * 3 + 0], cpu.outputQueue[i * 3 + 1])
            let tileId = cpu.outputQueue[i * 3 + 2]

            if pos == (-1, 0): #score
                score = tileId
            elif tileId == 0: #blank
                grid[pos[1]][pos[0]] = ' '
            elif tileId == 1: #wall
                grid[pos[1]][pos[0]] = '|'
            elif tileId == 2: # block
                grid[pos[1]][pos[0]] = '#'
            elif tileId == 3: # paddle
                paddlePos = pos
                grid[pos[1]][pos[0]] = '='
            elif tileId == 4: # ball
                ballPos = pos
                grid[pos[1]][pos[0]] = 'O'
        cpu.outputQueue.clear()

        if verbose:
            echo "Score: ", score, "\tBall pos: ", ballPos, "\tPaddle pos: ", paddlePos
            for r in 0..<GridRows:
                var rowString = ""
                for c in 0..<GridCols:
                    rowString.add(grid[r][c])
                echo rowString
            echo ""
            os.sleep(30)

        if didHalt:
            break

        # Move the paddle to always be under the ball
        cpu.inputQueue.addLast(if paddlePos[0] > ballPos[0]: -1 elif paddlePos[0] < ballPos[0]: 1 else: 0)

    echo "Part 2: ", score


proc solve*(inputs_str: seq[string]): void=
    let data = inputs_str[0].split(',').map(parseInt)

    part1(data);
    part2(data, true);
