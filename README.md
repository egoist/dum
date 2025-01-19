<br><br><br>

<p align="center">
<img src="https://mystickermania.com/cdn/stickers/games/among-us-lime-character-dum.svg" width="200">
<br><br>
<strong>dum</strong> replaces <code>npm run</code> and <code>npx</code>. <br>Instead of waiting 200ms for your npm client to start, it will start immediately.
<br>
<strong>💛 You can help the author become a full-time open-source maintainer by <a href="https://github.com/sponsors/egoist">sponsoring him on GitHub</a>.</strong>
</p>

<br>

<p align="center">
<img width="500" alt="CleanShot 2021-11-20 at 15 23 54@2x" src="https://user-images.githubusercontent.com/8784712/142718353-6e6f8327-c27e-404a-866e-6d5af3567cbc.png"></p>

<br>

---

<br>

## How

This is written in Rust! (Or any compile-to-native language).

Benchmark (`hyperfine "dum foo" "npm run foo" --warmup 10`):

| Command       |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------ | ----------: | -------: | -------: | ----------: |
| `dum foo`     |  41.7 ± 1.2 |     39.8 |     44.6 |        1.00 |
| `npm run foo` | 333.7 ± 2.0 |    330.0 |    336.0 | 8.01 ± 0.23 |

## Install

### Homebrew

```bash
brew install egoist/tap/dum
```

### Arch Linux AUR

```bash
yay -S dum
# or
paru -S dum
```

Check <https://aur.archlinux.org/packages/dum> for version info.

### Shell

```bash
curl -sSL https://bina.egoist.dev/egoist/dum | sh
```

### Cargo

```bash
cargo install dum
```

### Scoop

```shell
scoop install dum
```

### GitHub Releases

[Download a release manually](https://github.com/egoist/dum/releases) and move it to `/usr/local/bin` manually.

## Usage

`dum <npm_script|bin_script> [...args_to_forward]`: Run npm scripts or scripts in `node_modules/.bin`, like `yarn run`, `npm run`, `npx`.

If you want to pass flags to `dum` itself, like the `-c` flag to change directory, you should put it before the script name, like `dum -c another/directory script_name --forward some_flag`.

Examples:

```bash
dum some-npm-script

dum some-npm-script --flags will --be forwarded
# Like npx, but mush faster
dum some-npm-package-cli-name --flags will --be forwarded

# Change working directory
dum -c packages/sub-package build

# More
dum --help
```

### Install Packages

Dum is not a package manager yet, but we forward `install`, `add`, `remove` commands to the package manager you're currently using:

```bash
# Run `npm i` or `yarn` or `pnpm i` depending on the project
dum install # or `dum i`
# Like above but add packages
dum add react vue -D

dum remove react vue
```

We detect the package manager automatically by checking for lock files in the current directory. If no lock file is found, we ask you to select a package manager first.

## Limitations

- [package.json vars](https://docs.npmjs.com/cli/v8/using-npm/scripts#packagejson-vars) are not supported, I personally never used it, if you believe it's necessary, please [leave a comment here](https://github.com/egoist/dum/issues/2).

## Inspiration

I want to try and learn Rust so I made this. Inspired by [bun](https://bun.sh/).

## Development

```bash
cargo run -- <...args to test>
```

## Sponsors

[![sponsors](https://sponsors-images.egoist.dev/sponsors.svg)](https://github.com/sponsors/egoist)

## License

MIT &copy; [EGOIST](https://github.com/sponsors/egoist)
