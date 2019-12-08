import algorithm
import deques
import math
import sequtils
import strutils

type IntCodeComputer = object
    data: seq[int]
    ip: int
    inputQueue: Deque[int]
    outputQueue: Deque[int]

proc initCpu(data: seq[int], inputs: seq[int]): IntCodeComputer=
    var cpu = IntCodeComputer(data: data, inputQueue: initDeque[int](), outputQueue: initDeque[int]())
    for i in inputs:
        cpu.inputQueue.addLast(i)
    return cpu

# Runs the Intcode emulator until we either halt or block on input. Returns whether we hit a HALT instruction
proc runSimulation(cpu: var IntCodeComputer): bool =
    while cpu.data[cpu.ip] != 99:
        let opcode = cpu.data[cpu.ip] mod 100;
        let addressingModes: int = cpu.data[cpu.ip] div 100

        let getParam = proc (cpu: IntCodeComputer, param: int): int=
            let isImmediate = ((addressingModes div (10 ^ (param - 1))) mod 10) == 1
            if isImmediate:
                cpu.data[cpu.ip + param]
            else:
                cpu.data[cpu.data[cpu.ip + param]]

        case opcode:
        of 1: # ADD
            cpu.data[cpu.data[cpu.ip + 3]] = getParam(cpu, 1) + getParam(cpu, 2)
            cpu.ip += 4

        of 2: # MUL
            cpu.data[cpu.data[cpu.ip + 3]] = getParam(cpu, 1) * getParam(cpu, 2)
            cpu.ip += 4

        of 3: # INPUT
            # Pause execution if we don't have enough input
            if cpu.inputQueue.len == 0:
                return false
            cpu.data[cpu.data[cpu.ip + 1]] = cpu.inputQueue.popFirst
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
            cpu.data[cpu.data[cpu.ip+3]] = if getParam(cpu, 1) < getParam(cpu, 2): 1 else: 0
            cpu.ip += 4
            
        of 8: # COMPE
            cpu.data[cpu.data[cpu.ip+3]] = if getParam(cpu, 1) == getParam(cpu, 2): 1 else: 0
            cpu.ip += 4

        else:
            echo "Unhandled opcode: ", opcode
            raise newException(Exception, "Unexpected opcode")

    return true

proc runAmplifiers(data: seq[int], inputsParam: seq[int]): int=
    var inputs = inputsParam

    while true:
        var cpus: seq[IntCodeComputer]
        for i in 0..4:
            cpus.add(initCpu(data, @[inputs[i]]))
        cpus[0].inputQueue.addLast(0)

        var isDone = false
        while not isDone:
            for i in 0..4:
                let didHalt = runSimulation(cpus[i])
                if i == 4 and didHalt:
                    isDone = true
                    break

                # Always run a feedback loop if our final amplifier hasn't halted
                if cpus[i].outputQueue.len > 0:
                    cpus[(i+1) mod 5].inputQueue.addLast(cpus[i].outputQueue.popFirst)

        result = max(result, cpus[4].outputQueue.peekLast())
        if inputs.nextPermutation() == false:
            break

proc solve*(inputs_str: seq[string]): void=
    let data = inputs_str[0].split(',').map(parseInt)
    echo "Part 1: ", runAmplifiers(data, @[0, 1, 2, 3, 4])
    echo "Part 2: ", runAmplifiers(data, @[5, 6, 7, 8, 9])
