module Accumulate

let accumulate (func: 'a -> 'b) (input: 'a list): 'b list =
    let rec rmap f acc list =
        match list with
        | [] -> List.rev acc
        | head :: tail ->
            tail |> rmap f ((f head) :: acc)

    rmap func [] input
