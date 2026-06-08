# UxPlay GUI - Interfaz Gráfica para UxPlay

Esta es una interfaz gráfica creada con **Rust + Tauri** para controlar UxPlay en Windows.

## Requisitos Previos

### 1. Compilar UxPlay para Windows

Sigue las instrucciones oficiales del repositorio de UxPlay:

```bash
# En Windows con MSYS2 y MinGW-64
pacman -S mingw-w64-x86_64-cmake mingw-w64-x86_64-gcc mingw-w64-x86_64-libplist mingw-w64-x86_64-gstreamer mingw-w64-x86_64-gst-plugins-base mingw-w64-x86_64-gst-plugins-good mingw-w64-x86_64-gst-plugins-bad mingw-w64-x86_64-gst-plugins-ugly mingw-w64-x86_64-openssl mingw-w64-x86_64-libavif mingw-w64-x86_64-dash-static mingw-w64-x86_64-libpsl mingw-w64-x86_64-libssh git

git clone https://github.com/FDH2/UxPlay.git
cd UxPlay
mkdir build && cd build
cmake -G "MSYS Makefiles" ..
make
make install
```

El ejecutable `uxplay.exe` se instalará en `/usr/local/bin/` de MSYS2.

### 2. Instalar Rust

Si no tienes Rust instalado:

```bash
# En Windows, descarga e instala desde:
# https://rustup.rs/
# O usa winget:
winget install Rustlang.Rust.GNU
```

### 3. Instalar Node.js

Descarga e instala desde: https://nodejs.org/

O usa winget:
```bash
winget install OpenJS.NodeJS.LTS
```

## Compilar la GUI

### Pasos de compilación:

1. **Instalar dependencias de Node.js:**
   ```bash
   cd uxplay-gui
   npm install
   ```

2. **Compilar la aplicación Tauri:**
   ```bash
   npm run tauri build
   ```

3. **Encontrar el ejecutable:**
   
   Los archivos compilados se encontrarán en:
   - `src-tauri/target/release/uxplay-gui.exe` (ejecutable principal)
   - `src-tauri/target/release/bundle/msi/` (instalador MSI)
   - `src-tauri/target/release/bundle/nsis/` (instalador NSIS)

## Uso de la GUI

1. Asegúrate de que `uxplay.exe` esté instalado y disponible en el PATH
2. Ejecuta `UxPlay GUI.exe`
3. Configura las opciones:
   - **Device Name**: Nombre del dispositivo (por defecto: UxPlay)
   - **Port**: Puerto opcional
   - **Verbose mode**: Modo detallado para logs
   - **No Video**: Solo audio (recomendado para mayor compatibilidad)
4. Haz clic en **Start UxPlay**
5. Desde tu dispositivo Apple (iPhone/iPad/Mac), busca "UxPlay" en AirPlay
6. Para detener, haz clic en **Stop UxPlay**

## Características

- ✅ Interfaz moderna y limpia
- ✅ Control completo de UxPlay
- ✅ Logs en tiempo real
- ✅ Indicador de estado
- ✅ Configuración de parámetros principales
- ✅ Compatible con Windows 10/11

## Estructura del Proyecto

```
uxplay-gui/
├── src/                    # Frontend HTML/CSS/JS
│   └── index.html
├── src-tauri/              # Backend Rust + configuración Tauri
│   ├── src/
│   │   └── main.rs         # Lógica principal
│   ├── icons/              # Iconos de la aplicación
│   ├── Cargo.toml          # Dependencias Rust
│   ├── tauri.conf.json     # Configuración Tauri
│   └── build.rs            # Script de compilación
├── package.json            # Dependencias Node.js
└── README.md               # Este archivo
```

## Comandos Disponibles

Desde el backend Rust, la GUI expone estos comandos:

- `start_uxplay`: Inicia el servidor UxPlay con los parámetros configurados
- `stop_uxplay`: Detiene el servidor UxPlay
- `get_status`: Obtiene el estado actual (Running/Stopped)

## Solución de Problemas

### UxPlay no se encuentra
Asegúrate de que `uxplay.exe` esté en el PATH del sistema o en la misma carpeta que la GUI.

### Error de permisos
Ejecuta la GUI como administrador si hay problemas de red.

### No aparece en AirPlay
Verifica que:
- El firewall permite conexiones entrantes
- La red está configurada como privada
- Los puertos UDP 5353 (mDNS) están abiertos

## Licencia

Este proyecto GUI utiliza la misma licencia que UxPlay (GPL v3).
