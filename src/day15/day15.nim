import deques
import math
import sequtils
import sets
import strutils
import tables

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

type Pos = tuple
        x, y: int

proc movePos(pos: Pos, direction: int): Pos=
    if direction == 1: #north
        (pos.x, pos.y + 1)
    elif direction == 2: #south
        (pos.x, pos.y - 1)
    elif direction == 3: #west
        (pos.x - 1, pos.y)
    elif direction == 4: #east
        (pos.x + 1, pos.y)
    else:
        raise newException(Exception, "Unexpected direction")

proc oppositeDirection(direction: int): int=
    if direction == 1: #north
        2
    elif direction == 2: #south
        1
    elif direction == 3: #west
        4
    elif direction == 4: #east
        3
    else:
        raise newException(Exception, "Unexpected direction")

proc explore(cpu: var IntCodeComputer, map: var Table[Pos, int], pos: Pos): void

proc tryMove(cpu: var IntCodeComputer, map: var Table[Pos, int], pos: Pos, direction: int): void=
    let newPos = movePos(pos, direction)
    if map.hasKey(newPos):
        return

    cpu.inputQueue.addLast(direction)
    discard cpu.runSimulation()
    let result = cpu.outputQueue.popFirst()

    map[newPos] = result

    if result > 0:
        explore(cpu, map, newPos)

        # Return to previous spot
        cpu.inputQueue.addLast(oppositeDirection(direction))
        discard cpu.runSimulation()
        cpu.outputQueue.popFirst()


proc explore(cpu: var IntCodeComputer, map: var Table[Pos, int], pos: Pos): void=
    tryMove(cpu, map, pos, 1) #north
    tryMove(cpu, map, pos, 2) #south
    tryMove(cpu, map, pos, 3) #west
    tryMove(cpu, map, pos, 4) #east

proc bfs(map: Table[Pos, int], start: Pos, dest: Pos): int=
    var queue = initDeque[tuple[pos: Pos, steps: int]]()
    queue.addLast((start, 0))

    var visited = initHashSet[Pos]()
    var maxSteps = 0

    while queue.len() > 0:
        let (pos, steps) = queue.popFirst()
        if pos == dest:
            return steps
        elif map.getOrDefault(pos) == 0:
            continue
        elif visited.contains(pos):
            continue

        maxSteps = max(maxSteps, steps)
        visited.incl(pos)
        
        queue.addLast((movePos(pos, 1), steps + 1))
        queue.addLast((movePos(pos, 2), steps + 1))
        queue.addLast((movePos(pos, 3), steps + 1))
        queue.addLast((movePos(pos, 4), steps + 1))

    # Return the total depth explored before not finding our target (so we can easily solve part 2)
    return maxSteps


proc printMap(map: Table[Pos, int]): void=
    var minX, minY = high(int) 
    var maxX, maxY = low(int)

    for pos in map.keys:
        minX = min(minX, pos.x)
        maxX = max(maxX, pos.x)
        minY = min(minY, pos.y)
        maxY = max(maxY, pos.y)

    for y in minY..maxY:
        var line = ""
        for x in minX..maxX:
            if (x, y) == (0, 0):
                line.add("*")
            elif map.hasKey((x, y)):
                let ch = map[(x, y)]
                if ch == 0:
                    line.add("#")
                elif ch == 2:
                    line.add("O")
                else:
                    line.add(" ")
            else:
                line.add("-")

        echo line

proc solve*(inputs_str: seq[string]): void=
    let data = inputs_str[0].split(',').map(parseInt)
    var cpu = initCpu(data, @[])
    discard cpu.runSimulation()

    var map = {(0, 0): 1}.toTable
    explore(cpu, map, (0, 0))

    var oxygenPos = (0, 0)
    for pos, tile in map.pairs:
        if tile == 2:
            oxygenPos = pos

    printMap(map)
    echo "Part 1: ", bfs(map, (0, 0), oxygenPos)
    echo "Part 2: ", bfs(map, oxygenPos, (999, 999))