
<h1 align="center">🐉 dndutils</h1>

<p align="center">
  <strong>A fast, interactive CLI tool for Dungeons & Dragons 5e, written in Rust.</strong>
</p>

<p align="center">
  <a href="https://github.com/toyo54/dndutils/releases">
    <img src="https://img.shields.io/github/v/release/toyo54/dndutils?logo=github&style=flat-square" alt="GitHub Release">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square" alt="License: MIT">
  </a>
</p>

---

`dndutils` is a simple helper for common dnd tasks like creating characters and rolling dices

## ✨ Features

* **🎲 Quick Dice Rolling:** Roll standard D&D dice (D4 to D100) with optional modifiers, safely handling extreme rolls and negative modifiers.
* **🧙‍♂️ Interactive Character Creator:** A step-by-step terminal wizard to build your character, featuring:
  * Interactive skill proficiency toggling (None, Proficient, Expertise, Half).
  * Real-time calculation of miscellaneous modifiers.
  * Multi-line input for rich inventory and backstory notes.
* **💾 Smart Saving:** Automatically saves your character data as a `.dndc` file safely to your Desktop (or the current directory as a fallback).

## 🟡 Limitations
Right now the creation tool is class and race agnostic.
This means that any actions and spells derived from class, race or items must be manually
written under the note section (can be done via the creation tool or manually as the `.dndc` format is just a plaintext file).

Future releases will support a more specific and in depth character creation process (like the one from Baldur's Gate 3) including: armor and weapons
proficiencies, spells and actions

## 🚀 Installation

You do not need to install Rust or compile anything to use `dndutils`. Just copy and paste the command for your operating system into your terminal to download the latest release.

### macOS and Linux
```bash
curl --proto '=https' --tlsv1.2 -LsSf [https://github.com/toyo54/dndutils/releases/latest/download/dndutils-installer.sh](https://github.com/toyo54/dndutils/releases/latest/download/dndutils-installer.sh) | sh

```

### Windows (PowerShell)

```powershell
irm [https://github.com/toyo54/dndutils/releases/latest/download/dndutils-installer.ps1](https://github.com/toyo54/dndutils/releases/latest/download/dndutils-installer.ps1) | iex

```

### Build from Source (For Rust Developers)

If you prefer to build from source and already have the Rust toolchain installed, you can use Cargo:

```bash
cargo install --git [https://github.com/toyo54/dndutils.git](https://github.com/toyo54/dndutils.git)

```

## 📖 Usage

`dndutils` uses standard subcommands. Running the tool without arguments displays the help menu.

### Character Management

Launch the interactive terminal UI to create a new character:

```bash
dndutils create

```

* **Navigation:** Use `Up/Down` arrows to select skills.
* **Proficiency:** Use `Left/Right` arrows to cycle between `[ ]` (None), `[P]` (Proficient), `[E]` (Expertise), and `[h]` (Half).
* **Modifiers:** Use `+` or `-` to adjust miscellaneous item/feat bonuses.

*Generated characters are saved to `~/Desktop/<CharacterName>.dndc`.*

### Dice Rolling

Roll any standard dice (`D4`, `D6`, `D8`, `D10`, `D12`, `D20`, `D100`) with an optional modifier.

```bash
# Basic roll
$ dndutils roll D20
Rolling D20...
Result: [ 14 ] = 14

# Roll with a positive modifier
$ dndutils roll d8 4
Rolling D8 +4...
Result: [ 6 ] +4 = 10

# Roll with a negative modifier
$ dndutils roll d20 -2
Rolling D20 -2...
Result: [ 12 ] -2 = 10

```

## 🤝 Contributing

Contributions, issues, and feature requests are always welcome! Feel free to open an issue or submit a pull request.

## 📝 License

This project is [MIT](https://www.google.com/search?q=LICENSE) licensed.
