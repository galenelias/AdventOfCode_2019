import deques
import math
import sequtils
import sets
import strutils
import tables
import algorithm

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

const ROWS = 49
const COLS = 57

proc buildGrid(output: var Deque[int]): seq[seq[char]]=
    # Add a buffer around the grid to make the algorithms easier
    setlen(result, ROWS + 2)
    for row in result.mitems:
        setlen(row, COLS+2)
        row.fill('.')

    for r in 1..ROWS:
        for c in 1..COLS:
            let ch = (char)output.popFirst()
            result[r][c] = ch

        discard output.popFirst() #newline

    discard output.popFirst() #newline

proc printGrid(grid: seq[seq[char]]): void=
    for row in grid:
        var rowStr = ""
        for ch in row:
            rowStr.add(ch)
        echo rowStr

type
    CompassDirections = enum
        cdNorth, cdEast, cdSouth, cdWest

type   
    Pos = tuple
        row: int
        col: int

proc countScaffolds(grid: seq[seq[char]]): int=
    for row in grid:
        for ch in row:
            if ch == '#':
                result += 1

proc isIntersection(grid: seq[seq[char]], pos: Pos): bool=
    result = grid[pos.row][pos.col] == '#' and grid[pos.row-1][pos.col] == '#' and grid[pos.row+1][pos.col] == '#' and grid[pos.row][pos.col-1] == '#' and grid[pos.row][pos.col+1] == '#'

proc part1(data: seq[int]): void=
    var cpu = initCpu(data, @[])
    discard cpu.runSimulation()

    let grid = buildGrid(cpu.outputQueue)

    echo "Output queue len: ", cpu.outputQueue.len()
    echo "Output queue: ", cpu.outputQueue

    var intersections = 0
    var alignments = 0

    for r in 1..<grid.len()-1:
        for c in 1..<grid[r].len()-1:
            if isIntersection(grid, (row: r, col: c)):
                intersections += 1
                alignments += r * c

    echo "Part 1: ", intersections, ", alignment = ", alignments
proc findBot(grid: seq[seq[char]]): tuple[pos: Pos, dir: CompassDirections]=
    for r in 0..<grid.len():
        for c in 0..<grid[r].len():
            let ch = grid[r][c]
            if ch == '<':
                return ((r, c), cdWest)
            elif ch == '>':
                return ((r, c), cdEast)
            elif ch == '^':
                return ((r, c), cdNorth)
            elif ch == '>':
                return ((r, c), cdSouth)

proc movePos(pos: Pos, dir: CompassDirections): Pos=
    case dir:
    of cdNorth:
        (pos.row - 1, pos.col)
    of cdSouth:
        (row: pos.row + 1, col: pos.col)
    of cdWest:
        (pos.row, pos.col - 1)
    of cdEast:
        (pos.row, pos.col + 1)

proc turnRight(dir: CompassDirections): CompassDirections=
    result = CompassDirections((ord(dir) + 1) mod 4)

proc turnLeft(dir: CompassDirections): CompassDirections=
    result = CompassDirections((ord(dir) + 3) mod 4)


proc enumeratePaths(grid: seq[seq[char]], oldPos: Pos, dir: CompassDirections, pathSoFar: string, inStepCount: int, inVisited: HashSet[Pos], totalNodes: int): seq[string]=
    # echo "Enumerate: pos=", oldPos, " path=", pathSoFar, " count=", inStepCount, " visited.len=", inVisited.len()

    # var newPathSoFar = pathSoFar
    # if soFarCount > 0:
    #     newPathSoFar.add($soFarCount)
    var newPos = oldPos
    var newStepCount = inStepCount
    var newVisited = inVisited

    newVisited.incl(newPos)

    while true:
        # Try turning left
        let leftDir = turnLeft(dir)
        let leftPos = movePos(newPos, leftDir)
        if grid[leftPos.row][leftPos.col] == '#' and not newVisited.contains(leftPos):
            result = concat(result, enumeratePaths(grid, leftPos, leftDir, pathSoFar & "," & $newStepCount & ",L", 1, newVisited, totalNodes))

        let rightDir = turnRight(dir)
        let rightPos = movePos(newPos, rightDir)
        if grid[rightPos.row][rightPos.col] == '#' and not newVisited.contains(rightPos):
            result = concat(result, enumeratePaths(grid, rightPos, rightDir, pathSoFar & "," & $newStepCount & ",R", 1, newVisited, totalNodes))

        newPos = movePos(newPos, dir)
        # Try to keep moving forward if we can
        if grid[newPos.row][newPos.col] != '#':
            break

        if newVisited.contains(newPos) and not isIntersection(grid, newPos):
            # echo "Break on contains: ", newPos, " steps=", newStepCount
            break

        newVisited.incl(newPos)
        newStepCount += 1
        # echo "BFS (step): ", newStepCount, " visitedLen=", newVisited.len()

        if newVisited.len() == totalNodes:
            # echo "Found solution: ", pathSoFar & $inStepCount
            return @[pathSoFar.substr(3) & "," & $inStepCount]

proc compressPath(path: string, index: int, patterns: seq[string], patternInProgress: string): seq[string]=
    if patterns.len() > 3:
        return @[]
    elif patternInProgress.len() > 20:
        return @[]

    if index == path.len():
        if patterns.len() == 3:
            return patterns
        else:
            return compressPath(path, index, concat(patterns, @[patternInProgress]), "")

    # Try extending current pattern
    result = compressPath(path, index + 1, patterns, patternInProgress & path[index])
    if result.len() > 0:
        return result

    # Try terminating current pattern
    if patternInProgress.len() > 1:
        result = compressPath(path, index + 1, concat(patterns, @[patternInProgress]), "")
        if result.len() > 0:
            return result

    

proc part2(dataIn: seq[int]): void=
    var data = dataIn
    data[0] = 2
    var cpu = initCpu(data, @[])
    discard cpu.runSimulation()

    let grid = buildGrid(cpu.outputQueue)
    var botPos = findBot(grid)
    let totalScaffolds = countScaffolds(grid)
    
    printGrid(grid)
    echo botPos

    var visitedNodes = initHashSet[Pos]();

    echo "Total scaffolds: ", totalScaffolds
    let paths = enumeratePaths(grid, botPos.pos, botPos.dir, "", 0, visitedNodes, totalScaffolds)

    # echo "Output queue len: ", cpu.outputQueue.len()
    # echo "Output queue: ", cpu.outputQueue

    echo "Paths: "
    for path in paths:
        echo path

        let compression = compressPath(path, 0, @[], "")
        if compression.len() == 3:
            echo "Found solution: ", compression
            break



proc solve*(inputs_str: seq[string]): void=
    let data = inputs_str[0].split(',').map(parseInt)

    # part1(data)
    # part2(data)

    let compression = compressPath("R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2", 0, @[], "")
    echo compression