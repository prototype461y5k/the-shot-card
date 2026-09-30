# AGENTS.md: project guidance for the Osaurus Code agent (The Shot Card)

Osaurus loads this file automatically when this folder is the chat's trusted folder.

## This project
- The Shot Card is a Tauri 2 app: React + TypeScript + Vite in `src/`, Rust in `src-tauri/`.
- Caches live inside the project and are already git-ignored under these names: `.cargo-home/`, `.rustup-home/`, `.npmcache/`. Use them:
  `export CARGO_HOME="$PWD/.cargo-home" RUSTUP_HOME="$PWD/.rustup-home" npm_config_cache="$PWD/.npmcache"`
- Rust tests: `cd src-tauri && cargo test --lib`
- Two tests read `test photo 2.jpg` from the folder above this repo (`../test photo 2.jpg`). Don't move or rename it.
- App bundle: `npm run tauri build -- --bundles app`, then make the DMG with the steps below.
- The DMG name follows `The-Shot-Card-v<version>-<short-description>-macOS-aarch64.dmg`. The version is in `package.json`.

## Rules
- Reply in English. Explain what you did in plain terms.
- Never invent command output, versions, file paths or hashes. Quote real tool output. If a step did not run, say so.
- Run `git status` before and after changes. Ask before `git push`, tags, or GitHub releases.

## Sandbox limits in trusted-folder mode
`shell_run` can only write inside this folder and temp directories. Keep caches inside the project and git-ignored:

```sh
export CARGO_HOME="$PWD/.cargo-home"
export npm_config_cache="$PWD/.npm-cache"
# Xcode: build in a temp folder, NOT inside the project. Kaan's Desktop is synced by
# iCloud, which adds Finder attributes that make codesign fail
# ("resource fork, Finder information, or similar detritus not allowed").
xcodebuild -scheme <Scheme> -configuration Release -derivedDataPath "$TMPDIR/DerivedData-<Scheme>" build
```

`.gitignore` should contain: `.cargo-home/`, `.npm-cache/`, `build/`, `*.dmg`.

## Building a DMG (tested in this sandbox)
`hdiutil create -srcfolder …` fails here ("Directory not empty"). Use this instead:

```sh
APP="path/to/MyApp.app"; NAME="MyApp"; VER="1.0.0"
rm -rf build/dmg-stage && mkdir -p build/dmg-stage
cp -R "$APP" build/dmg-stage/
ln -s /Applications build/dmg-stage/Applications
hdiutil makehybrid -hfs -hfs-volume-name "$NAME" -o build/tmp.dmg build/dmg-stage
hdiutil convert build/tmp.dmg -format UDZO -ov -o "build/$NAME-$VER.dmg"
hdiutil verify "build/$NAME-$VER.dmg"
shasum -a 256 "build/$NAME-$VER.dmg"
```

## npm / Tauri
Pass CLI flags after `--`: `npm run tauri build -- --bundles app`.

## Releases
Match the style of earlier release notes (`gh release view <previous-tag>`). Include the SHA-256 from the command above, copied verbatim.
