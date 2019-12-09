import deques
import math
import sequtils
import strutils

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

proc solve*(inputs_str: seq[string]): void=
    let data = inputs_str[0].split(',').map(parseInt)

    var cpu1 = initCpu(data, @[1])
    discard cpu1.runSimulation()
    echo "Part 1: ", cpu1.outputQueue.popFirst

    var cpu2 = initCpu(data, @[2])
    discard cpu2.runSimulation()
    echo "Part 2: ", cpu2.outputQueue.popFirst
