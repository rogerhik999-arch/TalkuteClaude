# Platform Setup Requirements

This document outlines the platform-specific setup requirements for Talkute on Windows, macOS, and Linux.

## Windows

### Permissions
- **Microphone Access**: Required for voice input
  - Windows Settings → Privacy → Microphone → Allow apps to access microphone
  - The app will request permission on first use

### Dependencies
- **Visual C++ Redistributable**: Required for Rust runtime
  - Download from Microsoft's website if not installed

### Hotkey Registration
- The app registers `Ctrl+Space` as a global hotkey by default
- This may conflict with other applications using the same hotkey
- Change the hotkey in Settings → General if needed

### System Tray
- Talkute runs in the system tray (notification area)
- Right-click the tray icon to access quick actions

### Text Injection
- Uses Windows API for keyboard simulation
- May require running as Administrator for some applications

---

## macOS

### Permissions
- **Microphone Access**: Required for voice input
  - System Preferences → Security & Privacy → Privacy → Microphone
  - Enable Talkute in the list

- **Accessibility Access**: Required for text injection
  - System Preferences → Security & Privacy → Privacy → Accessibility
  - Add Talkute to the allowed apps

### Dependencies
- **Xcode Command Line Tools**: Required for some Rust dependencies
  - Run `xcode-select --install` if not installed

### Hotkey Registration
- The app registers `Ctrl+Space` (or `Cmd+Space`) as a global hotkey
- This may conflict with Spotlight Search
- Change the hotkey in Settings if needed

### System Tray
- Talkute runs in the menu bar
- Click the icon to access quick actions

### Text Injection
- Uses macOS Accessibility APIs
- Requires Accessibility permission to be granted

---

## Linux

### Permissions
- **Microphone Access**: Required for voice input
  - PulseAudio or PipeWire must be configured
  - Check with `pactl info`

### Dependencies
- **GTK 3**: Required for system tray and UI
  - Ubuntu/Debian: `sudo apt install libgtk-3-dev`
  - Fedora: `sudo dnf install gtk3-devel`
  - Arch: `sudo pacman -S gtk3`

- **X11 Development Libraries**: Required for hotkey and text injection
  - Ubuntu/Debian: `sudo apt install libx11-dev libxtst-dev`
  - Fedora: `sudo dnf install libX11-devel libXtst-devel`
  - Arch: `sudo pacman -S libx11 libxtst`

### Hotkey Registration
- The app registers `Ctrl+Space` as a global hotkey
- Works with X11; Wayland support is limited

### System Tray
- Requires a system tray implementation (e.g., GNOME Shell with AppIndicator extension)
- On GNOME, install the "AppIndicator" extension

### Text Injection
- Uses X11 test extension for keyboard simulation
- Wayland users may need to switch to X11 session for full functionality

---

## Mobile (iOS/Android)

### iOS

### Permissions
- **Microphone Access**: Required for voice input
  - Add `NSMicrophoneUsageDescription` to Info.plist
  - The app will request permission on first use

- **Full Keyboard Access**: Required for keyboard extension
  - Settings → General → Keyboard → Keyboards → Add New Keyboard
  - Enable "Allow Full Access" for clipboard integration

### Setup
1. Install the main app from App Store
2. Go to Settings → General → Keyboard → Keyboards
3. Tap "Add New Keyboard" and select Talkute
4. Enable "Allow Full Access" for full functionality

### Android

### Permissions
- **Microphone Access**: Required for voice input
  - Add `RECORD_AUDIO` permission to AndroidManifest.xml
  - The app will request permission on first use

- **Input Method**: Required for keyboard service
  - Enable Talkute in Settings → System → Languages & input → Virtual keyboard

### Setup
1. Install the app from Play Store
2. Go to Settings → System → Languages & input → Virtual keyboard
3. Enable Talkute keyboard
4. Set as default keyboard or switch using keyboard picker

---

## Troubleshooting

### Common Issues

#### Microphone not working
- **Windows**: Check Privacy settings, ensure microphone is not disabled
- **macOS**: Check Security & Privacy settings, restart the app
- **Linux**: Check PulseAudio/PipeWire configuration

#### Hotkey not responding
- Check if another app is using the same hotkey
- Change the hotkey in Settings
- On Linux, ensure X11 is being used (Wayland has limited hotkey support)

#### Text not inserting
- **Windows**: Try running as Administrator
- **macOS**: Grant Accessibility permission in Security & Privacy
- **Linux**: Ensure X11 test extension is available

#### Tray icon not showing
- **Windows**: Check notification area settings
- **macOS**: Check menu bar extras settings
- **Linux**: Install AppIndicator extension for GNOME

### Getting Help
If issues persist, please:
1. Check the app logs (Help → View Logs)
2. Report issues at github.com/rogerhik999-arch/TalkuteClaude/issues