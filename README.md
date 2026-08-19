# The Shot Card

> Instagram posts made easy.

The Shot Card is a native macOS desktop application for photographers who want to turn a photograph and its technical camera metadata into a clean, Instagram-ready composition. It combines a source image, a selectable canvas ratio, a technical information block, and a controlled visual layout into a single exportable card.

## Transparency note

This project was **vibe-coded with Manus 1.6**. The application was developed through iterative natural-language collaboration, code generation, visual review, debugging, and manual validation. The project is published openly so that the implementation, design decisions, limitations, and build process can be inspected by others.

The project is not presented as a fully independent commercial-grade software product. It is an evolving open-source experiment and a practical photography utility. Contributions, bug reports, and review are welcome.

## What it does

The application runs locally on macOS. Photos are imported through a native file dialog or drag-and-drop. The interface provides a fixed preview workspace with zoom controls, photo-only mouse dragging, and trackpad gesture support. When the preview is zoomed beyond 100%, two-finger trackpad scrolling can pan the preview even when the pointer is over an empty canvas area.

A composition can include camera body, lens, focal length, aperture, shutter speed, and ISO information. EXIF data is not written into the form automatically during import. The user explicitly chooses **Fill from EXIF** when metadata should be read into the technical fields. The numeric technical fields accept numeric characters, while display units are added only to the rendered composition line.

The export pipeline is native Rust code. It supports PNG, JPEG, and TIFF output, 1×/2×/3× resolution multipliers, a native macOS save dialog, user-selected filenames and folders, and Finder reveal notifications after export.

## Public repository and releases

The source repository is public at [github.com/prototype461y5k/the-shot-card](https://github.com/prototype461y5k/the-shot-card). macOS builds and source archives are published under the repository’s [Releases](https://github.com/prototype461y5k/the-shot-card/releases) section. The Apple Silicon release asset is named `The-Shot-Card-v0.6.8-toast-hitarea-fix-macOS-aarch64.dmg`.

The project’s policy documents are available as [Privacy Policy](PRIVACY.md) and [Terms of Service](TERMS.md). They are practical project drafts describing the current local-only data flow and open-source distribution model; they should be reviewed by a qualified attorney before being relied on as formal legal documents.

### Release checksums

The SHA-256 checksum below corresponds to the DMG published in the v0.6.8 release. Compare the checksum after downloading to confirm that the local file matches the published artifact. Previous release assets remain available in the [Releases](https://github.com/prototype461y5k/the-shot-card/releases) archive.

| Asset | SHA-256 |
|---|---|
| `The-Shot-Card-v0.6.8-toast-hitarea-fix-macOS-aarch64.dmg` | `c65446787dff15855ed4903a69966ee1d982aa545bef5e61af862ab3d008cf25` |

On macOS or Linux, run `shasum -a 256 <downloaded-file>` or `sha256sum <downloaded-file>`. On Windows PowerShell, run `Get-FileHash .\\downloaded-file -Algorithm SHA256`.

## Main features

| Area | Included behavior |
|---|---|
| Import | Native macOS file picker, drag-and-drop, local image processing |
| Metadata | Native EXIF reading for supported JPEG files; single-photo and batch EXIF fill buttons |
| Canvas presets | 3:4, 4:5, 1:1, 1.91:1, and 4.74:1 ultra-wide |
| Preview | Fixed viewport, zoom up to 500%, FIT reset, photo drag, trackpad pan |
| Layout | Contain and Cover/Crop modes with ratio-aware spacing defaults |
| Technical fields | Camera body, lens, focal length, aperture, shutter, ISO |
| User lists | Add/remove camera and lens values; saved locally between sessions |
| Typefaces | Twelve bundled typefaces with no system-font dependency |
| Export | PNG lossless, JPEG quality 100, TIFF master, 1×/2×/3× resolution, local progress indicator, Finder reveal |
| Languages | English, Türkçe, Español, Deutsch, Français, Italiano |
| Privacy | Local processing; photos are not uploaded by the application |

## Layout defaults

The default 4:5 composition follows the supplied reference direction. It uses approximately 44 px side margins, a 120 px top reserve, a shared 992×1078 photo area on a 1080×1350 canvas, a 36 px photo-to-technical-information gap, and a 32 px bottom safety reserve. Other presets reuse the same spacing logic while adapting the photo geometry to their canvas ratio. The 4.74:1 preset uses a dedicated shallow-strip layout.

## Technical architecture

The application consists of a React and TypeScript frontend rendered with Vite, connected to a native Rust backend through Tauri 2.

| Layer | Technology | Responsibility |
|---|---|---|
| UI | React 18, TypeScript, Vite | Editor panels, preview, language state, local preferences |
| Native shell | Tauri 2 | macOS window, dialogs, filesystem bridge, packaging |
| Native backend | Rust | Image decoding, EXIF reading, compositing, text rendering, export |
| Image processing | Rust `image` crate with Lanczos3 resizing | Canvas composition and export resampling |
| Text rendering | `ab_glyph` with bundled fonts | Native technical metadata typography |
| EXIF | `kamadak-exif` | Reading supported JPEG metadata |

The frontend does not require an external server to perform its core workflow. The application is intended to process imported images locally on the user’s Mac.

## Requirements

The current release targets Apple Silicon Macs running macOS. The repository contains the source code and build configuration; the distributed DMG is an Apple Silicon build.

To build from source, install Node.js, Rust, and the Tauri prerequisites for macOS. Then run:

```bash
npm install
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

The generated macOS bundles are written under:

```text
src-tauri/target/release/bundle/
```

## macOS Gatekeeper and unsigned distribution

The distributed application is **not signed with an Apple Developer ID and is not notarized**. This is intentional for the current open-source development stage because the project does not have an Apple Developer ID signing identity. macOS may therefore display a security warning when the application is first opened.

The expected graphical approval path is:

1. Open the DMG and drag **The Shot Card.app** to Applications.
2. Try to open the application once.
3. Open **System Settings → Privacy & Security**.
4. In the Security section, choose **Open Anyway** for The Shot Card.
5. Confirm the macOS prompt and open the application again.

The DMG uses the standard Tauri packaging flow and includes an Applications shortcut. No terminal command should be required for the normal first-open approval flow. A future release can remove this warning only after the app is signed with an Apple Developer ID certificate and notarized by Apple.

## Source tree

```text
.
├── src/                  React and TypeScript frontend
├── src-tauri/            Rust backend and Tauri configuration
│   ├── src/lib.rs        Native commands and export pipeline
│   └── assets/           Bundled typefaces
├── public/               Static frontend assets
├── package.json          Frontend scripts and dependencies
└── README.md             Project documentation
```

## Native tests

The Rust backend includes acceptance-oriented tests covering the bundled fonts, EXIF reading, native image import, export code paths, required canvas dimensions, and Sigma lens-name cleanup. The current test suite contains seven passing tests in the development environment.

## Limitations and known considerations

The current distributed build is Apple Silicon-specific. EXIF behavior depends on metadata being present and readable in the imported file; metadata is not guaranteed for every image format or editing workflow. macOS Gatekeeper warnings are expected because the application is not notarized. Instagram will apply its own platform-side processing after upload, so the exported file is not a guarantee of the final Instagram delivery representation.

The project is actively evolving. UI translations, layout defaults, typography, native packaging, and gesture behavior may continue to change.

## License

This project is released under the [MIT License](LICENSE). The license permits use, copying, modification, publication, distribution, sublicensing, and sale of the software, subject to the conditions in the license file. The software is provided without warranty.

## Acknowledgements

The Shot Card was created as an experimental photography workflow tool with assistance from **Manus 1.6**. The project name, logo, interface, native export workflow, and technical layout system were iterated during the development process described above.
