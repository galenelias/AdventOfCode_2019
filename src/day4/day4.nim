import strutils
import sequtils

proc isValidPass1(pass: string): bool=
    var doubleNum = false
    for i in 0..<pass.len-1:
        if pass[i] == pass[i+1]:
            doubleNum = true

        if pass[i] > pass[i+1]:
            return false

    return doubleNum

proc isValidPass2(pass: string): bool=
    var doubleNum = false
    for i in 0..<pass.len-1:
        if pass[i] == pass[i+1] and (i == pass.len - 2 or pass[i] != pass[i+2]) and (i == 0 or pass[i] != pass[i-1]):
            doubleNum = true

        if pass[i] > pass[i+1]:
            return false

    return doubleNum

let inputs = stdin.readLine().split('-').map(parseInt)

var goodPasses1 = 0
var goodPasses2 = 0

for pass in inputs[0]..inputs[1]:
    if isValidPass1($pass):
        goodPasses1 += 1

    if isValidPass2($pass):
        goodPasses2 += 1

echo "Part 1: ", goodPasses1
echo "Part 2: ", goodPasses2