# UxPlay GUI - Windows Installation Guide

This guide explains how to compile UxPlay for Windows and then build the Rust+Tauri GUI application.

## Prerequisites

### 1. Install MSYS2
1. Download MSYS2 from https://www.msys2.org/
2. Run the installer and follow the instructions
3. After installation, launch "MSYS2 UCRT64" from the Start menu

### 2. Install UxPlay Dependencies in MSYS2
In the MSYS2 UCRT64 terminal, run:

```bash
pacman -Syu
# Close and reopen the terminal after this update completes

pacman -S mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-cmake mingw-w64-ucrt-x86_64-ninja mingw-w64-ucrt-x86_64-pkg-config
pacman -S mingw-w64-ucrt-x86_64-libplist mingw-w64-ucrt-x86_64-gstreamer mingw-w64-ucrt-x86_64-gst-plugins-base
pacman -S mingw-w64-ucrt-x86_64-gst-plugins-good mingw-w64-ucrt-x86_64-gst-plugins-bad mingw-w64-ucrt-x86_64-gst-plugins-ugly
```

### 3. Compile UxPlay
```bash
# Clone the repository
git clone https://github.com/FDH2/UxPlay.git
cd UxPlay

# Create build directory
mkdir build && cd build

# Configure with CMake
cmake -G Ninja -DCMAKE_BUILD_TYPE=Release ..

# Build
ninja

# Install (optional, copies to MSYS2 system directories)
ninja install
```

After compilation, you'll find `uxplay.exe` in the `build` directory. Copy it to a location in your PATH or add the build directory to your PATH.

### 4. Verify UxPlay Installation
Open a regular Command Prompt or PowerShell and run:
```cmd
uxplay --version
```

If it shows version information, UxPlay is correctly installed.

## Building the GUI Application

### 5. Install Rust
1. Download and run the Rust installer from https://rustup.rs/
2. Follow the installation prompts
3. Restart your terminal after installation

### 6. Install Node.js
Download and install Node.js LTS from https://nodejs.org/

### 7. Install Tauri Dependencies
Open PowerShell as Administrator and run:
```powershell
npm install -g @tauri-apps/cli
```

### 8. Build the GUI
Navigate to the `uxplay-gui` directory and run:

```bash
# Install frontend dependencies
npm install

# Build the application
npm run tauri build
```

The compiled application will be in `src-tauri/target/release/`.

## Using the Application

1. Launch `UxPlay GUI.exe`
2. Configure your settings:
   - **Device Name**: How your computer will appear on iOS/macOS devices
   - **Port**: Optional custom port (default is usually fine)
   - **Verbose mode**: Enable detailed logging
   - **Audio only**: Disable video streaming for better performance
   - **Fullscreen**: Enable fullscreen mode
3. Click "Start UxPlay"
4. On your iPhone/iPad/Mac, open Control Center and select Screen Mirroring
5. Choose your device name from the list
6. The stream should now appear on your computer

## Troubleshooting

### "Failed to start uxplay" error
- Ensure UxPlay is installed and in your system PATH
- Try running `uxplay` from Command Prompt to verify installation

### No audio/video
- Make sure you have the correct GStreamer plugins installed
- Try enabling/disabling the "Audio only" option

### Firewall issues
- Windows Firewall may block incoming AirPlay connections
- Allow `uxplay.exe` through Windows Firewall when prompted

## Alternative: Portable Setup

If you don't want to add UxPlay to your PATH:
1. Copy `uxplay.exe` to the same directory as `UxPlay GUI.exe`
2. The GUI will automatically find it

## Additional Resources

- UxPlay GitHub: https://github.com/FDH2/UxPlay
- Tauri Documentation: https://tauri.app/
- GStreamer Windows: https://gstreamer.freedesktop.org/download/
