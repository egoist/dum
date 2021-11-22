## Unreleased

- Fallback to run binaries in `node_modules/.bin/` when specified script doesn't exist in `package.json`.

## v0.1.9

- Add `remove` command, mirrors `npm remove` `yarn remove` and `pnpm remove`.
- Add `-c <dir>` flag to change working directory.

## v0.1.8

### Fixes

- Fetch `PATH` env at runtime.

## v0.1.7

- Add command `add`

## v0.1.6

- Forward args to `install` command.

## v0.1.5

- Alias script `t` to `test`, so `dum t` and `dum test` are equivalent.
- Add `install` command to automatically run `npm i`, `yarn` or `pnpm i` depending on the project.
