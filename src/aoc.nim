import os
from strutils import parseInt
import day1/day1
import day2/day2
import day3/day3
import day4/day4
import day5/day5
import day6/day6
import day7/day7
import day8/day8
import day9/day9
import day10/day10
import day11/day11
import day13/day13
import day14/day14
import day15/day15
import day16/day16

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
    of 6: day6.solve(inputs)
    of 7: day7.solve(inputs)
    of 8: day8.solve(inputs)
    of 9: day9.solve(inputs)
    of 10: day10.solve(inputs)
    of 11: day11.solve(inputs)
    of 13: day13.solve(inputs)
    of 14: day14.solve(inputs)
    of 15: day15.solve(inputs)
    of 16: day16.solve(inputs)
    else:
        echo "Unsupported day: ", paramStr(1)