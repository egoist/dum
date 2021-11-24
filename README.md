<br><br><br>

<p align="center">
<img src="https://mystickermania.com/cdn/stickers/games/among-us-lime-character-dum.svg" width="200">
<br><br>
<strong>dum</strong> replaces <code>npm run</code>. <br>Instead of waiting 200ms for your npm client to start, it will start immediately.
<br>
<strong>💛 You can help the author become a full-time open-source maintainer by <a href="https://github.com/sponsors/egois">sponsoring him on GitHub</a>.</strong>
</p>

<br>

<p align="center">
<img width="500" alt="CleanShot 2021-11-20 at 15 23 54@2x" src="https://user-images.githubusercontent.com/8784712/142718353-6e6f8327-c27e-404a-866e-6d5af3567cbc.png"></p>

<br>

---

<br>

## How

This is written in Rust! (Or any compile-to-native language).

## Install

If you are a Rust user:

```bash
cargo install dum
```

Or [download a release](https://github.com/egoist/dum/releases) and move it to `/usr/local/bin` manually.

### Homebrew

```bash
brew install bytesfriends/fun/dum
```

Or

```bash
brew tap bytesfriends/fun && brew install dum
```

PR welcome for adding a shell script so you can install `dum` with a single `curl` command.

## Usage

```bash
dum some-npm-script

dum some-npm-script --flags will --be forwareded

# Run `npm i` or `yarn` or `pnpm i` depending on the project
dum install # or `dum i`
# Like above but add packages
dum add react vue -D

# Change working directory
dum -c packages/sub-package build

# More
dum --help
```

## Limitations

- [package.json vars](https://docs.npmjs.com/cli/v8/using-npm/scripts#packagejson-vars) are not supported, I personally never used it, if you believe it's necessary, please [leave a comment here](https://github.com/egoist/dum/issues/2).

## Inspiration

I want to try and learn Rust so I made this. Inspired by [bun](https://bun.sh/).

## Sponsors

[![sponsors](https://sponsors-images.egoist.sh/sponsors.svg)](https://github.com/sponsors/egoist)

## License

MIT &copy; [EGOIST](https://github.com/sponsors/egoist)
