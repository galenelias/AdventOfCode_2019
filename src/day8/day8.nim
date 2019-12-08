import math
import sequtils

proc solve*(inputs_str: seq[string]): void=
    let input = toSeq(inputs_str[0])

    const LAYER_HEIGHT = 6
    const LAYER_WIDTH = 25
    const LAYER_SIZE = LAYER_HEIGHT * LAYER_WIDTH
    let layerCount = input.len div LAYER_SIZE

    let layers = input.distribute(layerCount)

    var fewestZeroes = LAYER_HEIGHT * LAYER_WIDTH
    var onesDigitsTimesTwoDigits = 0

    for layer in layers:
        let rows = layer.distribute(LAYER_HEIGHT)
        let zeroes = rows.mapIt(it.filterIt(it == '0').len()).sum()

        if zeroes < fewestZeroes:
            fewestZeroes = zeroes
            let ones = rows.mapIt(it.filterIt(it == '1').len()).sum()
            let twos = rows.mapIt(it.filterIt(it == '2').len()).sum()
            onesDigitsTimesTwoDigits = ones * twos

    echo "Part 1: ", onesDigitsTimesTwoDigits

    echo "Part 2:"
    for row in 0..<LAYER_HEIGHT:
        var result = ""
        for col in 0..<LAYER_WIDTH:
            for layer in layers:
                let pixel = layer[row * LAYER_WIDTH + col] 
                if pixel == '0':
                    result.add(" ")
                    break
                elif pixel == '1':
                    result.add("#")
                    break

        echo result
