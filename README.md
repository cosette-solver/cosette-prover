# The Cosette Prover

This is a reimplementation of the Cosette prover in Rust aiming for high performance and better SQL feature coverage.
The theory behind the prover is described in [this paper](https://www.vldb.org/pvldb/vol11/p1482-chu.pdf).
It currently expect input generated from [this parser](https://github.com/cosette-solver/cosette-parser).

## Usage

After downloading the source, build the source with
```sh
cargo build --release
```
Currently the build-time dependency are `libclang` and header files for `z3`.
Also, a nightly Rust compiler toolchain is required.
The build should produce an executable in `target/release/`, or you can use `cargo` to run:
```sh
cargo run --release -- <inputs>
```
where the `<inputs>` are paths to input files or directories containing the files.
The current setup of the solver has the Z3 *and* CVC5 solver as a runtime dependency,
so you need to have both the `z3` *and* `cvc5` executable in the `PATH` when running Cosette.
The results will be simply printed out, with the name of each file and their result (provable/not provable).
You can set the environment variable to `RUST_LOG=info` when running and get a (very) verbose output.
This is useful for debugging as it prints out how expressions are simplified in the solver at various stages.

Then input files should be generated by the parser.
They are all in JSON format.
The directory `tests/RelOptRulesTest/` contains such files that are directly extracted from the [Calcite](https://calcite.apache.org/) project.
You may try out running these inputs by
```sh
cargo run --release -- tests/RelOptRulesTest/
```
WARNING: many test cases in this folder contain features that we haven't support yet.
Now ~200 out of all ~380 cases are actually provable.

## Feature coverage

### Supported features
- Basic `SELECT-FROM-WHERE` queries
- Set operations (`UNION`, `UNION ALL`, `EXCEPT`, and `EXCEPT ALL`)
- Joins (`INNER JOIN`, `LEFT`/`RIGHT`/`FULL` `OUTER JOIN`, `SEMI`/`ANTI` `JOIN`, and correlated join)
- `DISTINCT`
- `VALUES`
- Aggretation (as uninterpreted functions)
- `ORDER BY` and `LIMIT` (as uninterpreted operators)
- Value operations with subquery (`IN` and `EXISTS`)
- Unique key constraint
- Arbitrary `CHECK` constraints

### Planned features
- Foreign key constraint
- Law of excluded middle ($A + \neg A = 1$)
- `INTERSECT` (but not `INTERSECT ALL`)

### Unsupported features
- Semantics of aggretations, such as understanding them as algebras over a monad
- Semantics of `ORDER BY` and `LIMIT` (requires temporarily modelling a table as list?)

## Reproducible environment

If fortunately you can use the Guix package manager with `direnv`, we provide all the necessary files to ensure maximal reproducibility of the exact development environment.
All the development dependencies are declared in `manifest.scm`, with packages drawn from `channels.scm`.
The `channels.lock` file is then derived from `channels.scm` to pin down channels to their exact Git commit hashes.
Finally, one can use `direnv` to automatically reproduce the declared environment after entering the project directory.

To run with this setup, add the following to `~/.config/direnv/direnvrc`.
```bash
use_guix-shell() {
	local profile=./.profile
	local channels=./channels.lock
	[ -L $profile ] && rm $profile
	if [ -f $channels ]; then
		eval "$(guix time-machine -C "$channels" -- shell -r "$profile" "$@" --search-paths)"
	else
		eval "$(guix shell -r "$profile" "$@" --search-paths)"
	fi
}
```
Now simply `cd` into the project directory, and `direnv` will try to build the development environment.
(NOTICE: For first-time use, you need to inspect and trust the `.envrc` file, and execute `direnv allow .` to proceed with the build)
