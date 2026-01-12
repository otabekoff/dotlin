# Dotlin VS Code Language Support (local)

This folder contains a minimal VS Code language extension for Dotlin (.lin) files.

## What is included

- `syntaxes/dotlin.tmLanguage.json` — TextMate grammar (adapted from Kotlin)
- `language-configuration.json` — comment/bracket/autoClose configuration
- `snippets/dotlin.json` — handy editor snippets (main, println, val/var, class, etc.)
- `package.json` — extension manifest registering the language and grammar

The extension icon files are in `images/` (icon.svg and icon.png). To avoid blurriness, generate a 128×128 PNG from the SVG and overwrite `images/icon.png` before packaging.

## Install locally

1. Open this folder in VS Code (`ide/vscode`).
2. Press `F5` to launch the Extension Development Host with the extension loaded.
3. Create a file with the `.lin` extension to see syntax highlighting and snippets.

Package and install locally:

```bash
cd ide/vscode
npm install
# generate a crisp 128x128 PNG icon from the SVG (ImageMagick)
npm run build:icon
# package the extension
npx vsce package
# install the produced .vsix
code --install-extension dotlin-language-0.1.0.vsix
```

If you have ImageMagick (recommended), the `build:icon` script now prefers it and preserves transparency:

```bash
npm run build:icon
# or directly with ImageMagick
magick convert -density 300 images/icon.svg -background none -resize 128x128 -gravity center -extent 128x128 images/icon.png
```

Fallbacks (if ImageMagick isn't installed):

```bash
rsvg-convert -w 128 -h 128 -o images/icon.png images/icon.svg
# or
inkscape images/icon.svg --export-type=png --export-filename=images/icon.png -w 128 -h 128
```

The `build:icon` script will try ImageMagick first, then `rsvg-convert`, and finally `inkscape`.

## Publishing / CI

This extension is minimal and intended for local development. I can add a GitHub Actions workflow to run tests, package the extension with `vsce`, and publish metadata if you'd like.
