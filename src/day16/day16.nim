import sequtils
import strutils
import tables
import algorithm

proc calcDigit(input: string, inputOffset: int): int=
    const pattern = @[0, 1, 0, -1]

    for i in 0..<input.len():
        let x = ((i + 1) div (inputOffset + 1)) mod pattern.len()
        result += pattern[x] * (ord(input[i]) - ord('0'))

    result = result mod 10
    if result < 0:
        result *= -1


proc fft(val: string): string=
    result = val
    for step in 0..<100:
        echo "FFT: ", step, " (", val.len(), ")"
        var temp = ""
        for i in 0..<result.len():
            temp.add($calcDigit(result, i))
        result = temp


proc solve*(inputs_str: seq[string]): void=
    let part1_str = fft(inputs_str[0])
    echo "Part 1: ", part1_str.substr(0, 7)

    var val2 = ""
    for i in 0..<10000:
        val2.add(inputs_str[0])

    let part2_str = fft(val2)
    echo part2_str.substr(0, 7)
