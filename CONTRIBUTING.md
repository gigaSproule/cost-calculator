# Setup local development

## Install GTK4

### Linux

#### Arch

```bash
sudo pacman -S gtk4 base-devel libadwaita
```

#### Debian

```bash
sudo apt install libgtk-4-dev build-essential libadwaita-1-dev
```

#### Fedora

```bash
sudo dnf install gtk4-devel gcc libadwaita-devel
```

### MacOS

```bash
brew install gtk4 libadwaita
```

### Windows

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools
winget install --id MSYS2.MSYS2
rustup default stable-msvc
python -m pip install --user pipx
python -m pipx ensurepath
# Open new powershell to get pipx in the path
pipx install gvsbuild
gvsbuild build gtk4 libadwaita librsvg
# Open new powershell as administrator
[Environment]::SetEnvironmentVariable("Path", "C:\gtk-build\gtk\x64\release\bin;" + $env:Path, [System.EnvironmentVariableTarget]::Machine)
[Environment]::SetEnvironmentVariable("LIB", "C:\gtk-build\gtk\x64\release\lib;" + $env:LIB, [System.EnvironmentVariableTarget]::Machine)
[Environment]::SetEnvironmentVariable("INCLUDE", "C:\gtk-build\gtk\x64\release\include;C:\gtk-build\gtk\x64\release\include\cairo;C:\gtk-build\gtk\x64\release\include\glib-2.0;C:\gtk-build\gtk\x64\release\include\gobject-introspection-1.0;C:\gtk-build\gtk\x64\release\lib\glib-2.0\include;" + $env:INCLUDE, [System.EnvironmentVariableTarget]::Machine)
```
