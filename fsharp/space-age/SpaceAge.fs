module SpaceAge
open System


[<Literal>]
let STD_YEAR_IN_S = 31557600.0

type Planet =
    | Mercury
    | Venus
    | Earth
    | Mars
    | Jupiter
    | Saturn
    | Uranus
    | Neptune
    member x.YearLength() =
        match x with
        | Mercury -> 0.2408467 * STD_YEAR_IN_S
        | Venus -> 0.61519726 * STD_YEAR_IN_S
        | Earth -> STD_YEAR_IN_S
        | Mars -> 1.8808158 * STD_YEAR_IN_S
        | Jupiter -> 11.862615 * STD_YEAR_IN_S
        | Saturn -> 29.447498 * STD_YEAR_IN_S
        | Uranus -> 84.016846 * STD_YEAR_IN_S
        | Neptune -> 164.79132 * STD_YEAR_IN_S


let age (planet: Planet) (seconds: int64): float =
    float seconds / planet.YearLength()

