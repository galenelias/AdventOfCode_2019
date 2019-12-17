import strutils

proc charToInt(ch: char): int=
    result = ord(ch) - ord('0')

proc calcDigit(input: string, inputOffset: int): int=
    const pattern = @[0, 1, 0, -1]

    for i in 0..<input.len():
        let x = ((i + 1) div (inputOffset + 1)) mod pattern.len()
        result += pattern[x] * charToInt(input[i])

    result = result mod 10
    if result < 0:
        result *= -1


proc fft(val: string): string=
    result = val
    for step in 0..<100:
        var temp = ""
        for i in 0..<result.len():
            temp.add($calcDigit(result, i))
        result = temp

# Optimized fft which only computes the last half of the number, since this can be done extremely efficiently
proc fft2(val: string): string=
    result = val
    for step in 0..<100:
        var sum = 0
        for i in 0..<result.len() div 2:
            sum = (sum + charToInt(result[result.len() - 1 - i])) mod 10
            result[result.len() - 1 - i] = ($sum)[0]

proc solve*(inputs_str: seq[string]): void=
    let part1 = fft(inputs_str[0])
    echo "Part 1: ", part1.substr(0, 7)

    var val2 = ""
    for i in 0..<10000:
        val2.add(inputs_str[0])

    let offset = parseInt(inputs_str[0][0..<7])
    let part2 = fft2(val2)
    echo "Part 2: ", part2[offset..offset+7]
