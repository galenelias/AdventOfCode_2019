import sequtils
import strutils
import tables
import algorithm

type RecipePart = object
    chemical: string
    amount: int

type Recipe = object
    fromParts: seq[RecipePart]
    toPart: RecipePart

proc getOreToMakeFuel(recipes: seq[Recipe], desiredFuel: int): int=
    var materials = {"FUEL": desiredFuel}.toTable

    for recipe in recipes:
        # We're guaranteed to always have the materials, since due to the most constrained ordering of the recipes
        let currentStock = materials[recipe.toPart.chemical]
        let conversionFactor = (currentStock + recipe.toPart.amount - 1) div recipe.toPart.amount # round up

        # Converting 'mat' to 'recipe.fromParts' with 'conversionFactor' multiplier
        for comp in recipe.fromParts:
            let currentAmount = materials.getOrDefault(comp.chemical)
            materials[comp.chemical] = currentAmount + conversionFactor * comp.amount
            materials.del(recipe.toPart.chemical)

    return materials["ORE"]

# Run our 'OreForFuel' process, binary searching on the fuel amount which can be made with 1 trillion ore
proc part2(recipes: seq[Recipe]): void=
    let targetOre = 1000000000000
    var highAmt = 1

    # First get a lower/upper bound by just iterating by orders of magnitude
    while getOreToMakeFuel(recipes, highAmt) < targetOre:
        highAmt *= 10

    var lowAmt = highAmt div 10

    # Then binary search on remaining ORE requirement
    while lowAmt < highAmt:
        let mid = (lowAmt + highAmt) div 2
        let res = getOreToMakeFuel(recipes, mid)

        if res < targetOre:
            lowAmt = mid + 1
        else:
            highAmt = mid

    echo "Part 2: ", lowAmt - 1

# We want our conversions to be sorted by hardest to make ingredient first
# since this is effectively most constrained. This will ensure that we add in
# the least amount of extra materials when our ratios don't exactly line up
proc sortRecipesByConstraint(recipes: var seq[Recipe]): void=
    # Figure our the 'rank' by propagating the ranks through the recipes.
    # A recipe output will have a rank 1 higher than the highest input material
    # Do this brute force with multiple iterations, as being smart is too much work
    var matRankings = initTable[string, int]()
    matRankings["ORE"] = 1
    while matRankings.len() < recipes.len() + 1:
        for recipe in recipes:
            var maxRank = 0
            var missingMat = false

            for part in recipe.fromParts:
                if not matRankings.hasKey(part.chemical):
                    missingMat = true
                    break
                else:
                    maxRank = max(maxRank, matRankings[part.chemical])

            if not missingMat:
                matRankings[recipe.toPart.chemical] = maxRank + 1

    # Sort descending by rank, so 'hardest' recipe is first
    sort(recipes, proc (a, b: Recipe): int= matRankings[a.toPart.chemical] - matRankings[b.toPart.chemical], SortOrder.Descending)

proc solve*(inputs_str: seq[string]): void=
    let parts = inputs_str.mapIt(it.split(" => ").mapIt(it.split(", ").mapIt(it.split(' '))))

    var recipes = parts.map( proc(it: seq[seq[seq[string]]]): Recipe=
        Recipe(
            fromParts: it[0].mapIt(RecipePart(amount: parseInt(it[0]), chemical: it[1])),
            toPart: RecipePart(amount: parseInt(it[1][0][0]), chemical: it[1][0][1]))
    )

    sortRecipesByConstraint(recipes)
 
    echo "Part 1: ", getOreToMakeFuel(recipes, 1)
    part2(recipes)