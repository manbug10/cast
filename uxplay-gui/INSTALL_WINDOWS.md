# Guía de Instalación para Windows

## Paso 1: Instalar MSYS2

1. Descarga MSYS2 desde https://www.msys2.org/
2. Ejecuta el instalador y sigue las instrucciones
3. Abre MSYS2 UCRT64 (no MSYS2 MSYS)

## Paso 2: Instalar dependencias de UxPlay

En la terminal de MSYS2 UCRT64, ejecuta:

```bash
pacman -Suy
pacman -S mingw-w64-ucrt-x86_64-cmake mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-libplist mingw-w64-ucrt-x86_64-gstreamer mingw-w64-ucrt-x86_64-gst-plugins-base mingw-w64-ucrt-x86_64-gst-plugins-good mingw-w64-ucrt-x86_64-gst-plugins-bad mingw-w64-ucrt-x86_64-gst-plugins-ugly mingw-w64-ucrt-x86_64-openssl mingw-w64-ucrt-x86_64-libavif mingw-w64-ucrt-x86_64-dash-static mingw-w64-ucrt-x86_64-libpsl mingw-w64-ucrt-x86_64-libssh git
```

## Paso 3: Compilar UxPlay

```bash
git clone https://github.com/FDH2/UxPlay.git
cd UxPlay
mkdir build && cd build
cmake -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=Release ..
mingw32-make
mingw32-make install
```

Verifica que se haya instalado:
```bash
which uxplay
uxplay --version
```

## Paso 4: Instalar Rust para Windows

Opción A: Usando winget (recomendado)
```powershell
winget install Rustlang.Rust.GNU
```

Opción B: Descargando desde rustup.rs
1. Ve a https://rustup.rs/
2. Descarga `rustup-init.exe`
3. Ejecuta el instalador con las opciones por defecto

## Paso 5: Instalar Node.js

Opción A: Usando winget
```powershell
winget install OpenJS.NodeJS.LTS
```

Opción B: Descargando desde nodejs.org
1. Ve a https://nodejs.org/
2. Descarga la versión LTS
3. Ejecuta el instalador

## Paso 6: Compilar la GUI

Abre PowerShell o CMD en la carpeta `uxplay-gui`:

```bash
# Instalar dependencias
npm install

# Compilar la aplicación
npm run tauri build
```

## Paso 7: Ejecutar la aplicación

Los archivos compilados estarán en:
- `src-tauri/target/release/uxplay-gui.exe` - Ejecutable portable
- `src-tauri/target/release/bundle/msi/*.msi` - Instalador MSI
- `src-tauri/target/release/bundle/nsis/*.exe` - Instalador NSIS

### Opción recomendada: Crear un instalador

El instalador NSIS incluye todo lo necesario y registra la aplicación en Windows.

## Solución de Problemas Comunes

### Error: "uxplay no se encuentra"

Asegúrate de que MSYS2 esté en el PATH o copia `uxplay.exe` a:
- `C:\Windows\System32\`
- O a la misma carpeta donde está `uxplay-gui.exe`

### Error de compilación de Rust

Actualiza Rust:
```bash
rustup update
```

### Error de Node.js

Reinstala Node.js y limpia npm:
```bash
npm cache clean --force
rm -rf node_modules
npm install
```

### La aplicación no aparece en AirPlay

1. Verifica que el Firewall de Windows permita la aplicación
2. Asegúrate de que la red esté configurada como "Privada"
3. Reinicia el servicio mDNS si es necesario

## Notas Importantes

- UxPlay requiere que los puertos UDP 5353 (mDNS) estén abiertos
- Para video, asegúrate de tener codecs compatibles instalados
- La primera conexión puede tardar unos segundos en establecerse
