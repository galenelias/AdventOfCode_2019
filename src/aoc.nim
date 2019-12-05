import os
from strutils import parseInt
import day1/day1
import day2/day2
import day3/day3
import day4/day4
import day5/day5

if paramCount() == 0:
    echo "Usage: ", paramStr(0), " <day_number>"
else:
    var inputs: seq[string]
    while not endoffile(stdin): 
        inputs.add(stdin.readLine())

    case parseInt(paramStr(1)):
    of 1: day1.solve(inputs)
    of 2: day2.solve(inputs)
    of 3: day3.solve(inputs)
    of 4: day4.solve(inputs)
    of 5: day5.solve(inputs)
    else:
        echo "Unsupported day: ", paramStr(1)