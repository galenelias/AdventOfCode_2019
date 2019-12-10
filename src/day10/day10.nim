import math
import sets
import algorithm

type
    Asteroid = tuple
      row: int
      col: int

# Compute whether or not any asteroids block the sight from asteroid1 to asteroid2 by
# walking the vector between them, and finding any intersecting asteroids
proc canSee(asteroid1: Asteroid, asteroid2: Asteroid, asteroidField: HashSet[Asteroid]): bool=
    if asteroid1 == asteroid2:
        return false

    let delta_r = asteroid2.row - asteroid1.row
    let delta_c = asteroid2.col - asteroid1.col

    var dr = 0
    var dc = 0

    if delta_r == 0:
        dr = 0
        dc = delta_c div abs(delta_c)
    elif delta_c == 0:
        dc = 0
        dr = delta_r div abs(delta_r)
    else:
        let gcd = gcd(delta_r, delta_c)
        dr = delta_r div gcd
        dc = delta_c div gcd

    let steps = if dr != 0: delta_r div dr else: delta_c div dc
    for i in 1..<steps:
        if asteroidField.contains((asteroid1.row + dr * i, asteroid1.col + dc * i)):
            return false

    return true


proc getVisibleAsteroids(fromAsteroid: Asteroid, allAsteroids: HashSet[Asteroid]): seq[Asteroid]=
    for otherAsteroid in allAsteroids:
        if canSee(fromAsteroid, otherAsteroid, allAsteroids):
            result.add(otherAsteroid)


proc solve*(inputs_str: seq[string]): void=
    var asteroids = initHashSet[Asteroid]()
    for row in 0..<inputs_str.len():
        for col in 0..<inputs_str[row].len():
            if inputs_str[row][col] == '#':
                asteroids.incl((row, col))

    var stationLocation: Asteroid
    var mostVisibleAsteroids = 0

    for asteroid in asteroids:
        let visibleAsteroids = getVisibleAsteroids(asteroid, asteroids)
        if visibleAsteroids.len() > mostVisibleAsteroids:
            mostVisibleAsteroids = visibleAsteroids.len()
            stationLocation = asteroid

    echo "Part 1: ", mostVisibleAsteroids, " (Location = ", stationLocation, ")"

    const TargetAsteroid = 200
    var asteroidsDestroyed = 0

    while true:
        var visibleAsteroids = getVisibleAsteroids(stationLocation, asteroids)
        if asteroidsDestroyed + visibleAsteroids.len() < TargetAsteroid:
            for asteroid in visibleAsteroids:
                asteroids.excl(asteroid)
            asteroidsDestroyed += visibleAsteroids.len()
        else:
            let getAngle = proc(a: Asteroid): float=
                result = arctan2(float((a.col - stationLocation.col)), float(-(a.row - stationLocation.row)))
                if result < 0:
                    result += TAU

            sort(visibleAsteroids, proc (a: Asteroid, b: Asteroid): int=
                if getAngle(a) < getAngle(b): -1 else: 1
            )
             
            let nthAsteroid = visibleAsteroids[TargetAsteroid - asteroidsDestroyed - 1]
            echo "Part 2: ", nthAsteroid.col * 100 + nthAsteroid.row
            break
