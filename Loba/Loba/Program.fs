open Loba.Loba

let input =
    "
take 42
take 42
take 42
"

let result = parse input

match result with
| Result.Ok res ->
    printfn $"{res}"
    let queryResult = execute res
    printfn $"{queryResult}"
| Result.Error err -> printfn $"{err}"
