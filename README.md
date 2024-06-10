# TypeView - Keystroke Visualizer

TypeView combines typing and viewing, providing a real-time visualization of your keystrokes. It's perfect for tutorials, presentations, or any scenario where you want to display your typing activity.

## Features

- **Real-time Visualization**: Shows your keystrokes with a clean overlay.
- **Mechanical Key Sound**: Simulates mechanical key sounds.
- **Lightweight**: Minimal performance impact.

## Installation

### macOS

If you encounter errors when opening the app, run:

```bash
xattr -rc /Applications/TypeView.app && codesign --force --deep --sign - /Applications/TypeView.app
```

This command resets the extended attributes of the app and re-signs it.

### Windows

Download the installer and follow the on-screen instructions to complete the installation.

## Usage

Open TypeView and start typing. Your keystrokes will be visualized in real-time.
