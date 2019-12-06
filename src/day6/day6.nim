import deques
import sequtils
import sets
import strutils
import tables

proc subOrbits(graph: Table[string, seq[string]], seed: string): int=
    result += graph.getOrDefault(seed).len()
    for subOrbit in graph.getOrDefault(seed):
        result += subOrbits(graph, subOrbit)

proc bfs(graph: Table[string, seq[string]], fromNode: string, toNode: string): int=
    var q = initDeque[tuple[n: string, s: int]]()
    var visited = initHashSet[string]()

    q.addLast((fromNode, 0))
    while q.len > 0:
        let (node, steps) = q.popFirst

        if visited.containsOrIncl(node):
            continue

        if node == toNode:
            return steps

        for child in graph[node]:
            q.addLast((child, steps + 1))

proc solve*(inputs_str: seq[string]): void=
    let pairs = inputs_str.mapIt(it.split(')'))

    var graph : Table[string, seq[string]]
    for pair in pairs:
        if not graph.hasKey(pair[0]):
            graph[pair[0]] = @[]
        graph[pair[0]].add(pair[1])

    var part1 = 0
    for key in graph.keys:
        part1 += subOrbits(graph, key)
    echo "Part 1: ", part1

    var neighbors = graph
    for key,value in graph:
        for child in value:
            if not neighbors.hasKey(child):
                neighbors[child] = @[]
            neighbors[child].add(key)

    echo "Part 2: ", bfs(neighbors, "YOU", "SAN") - 2
