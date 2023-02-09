# alww-hash

## Commands

| Command    | Syntax                         | Explanation                                                                 |
| ---------- | ------------------------------ | --------------------------------------------------------------------------- |
| cargo run  | `cargo run [arg] [param]`      | `Execute` the `arg` with `param` as the parameter                           |
| add/insert | `cargo run add/insert [param]` | `Add/Insert` the `param` to the database                                    |
| delete     | `cargo run delete [param]`     | `Delete` the `param` from the database                                      |
| get        | `cargo run get [param]`        | `Get` the `hash-key` for the given `param` given that it is in the database |
| print      | `cargo run print`              | `Printout` all entries in the database                                      |

## Example

`cargo run add hello`

## Database

All data is stored in the data.csv file.
This is non-negotiable.
