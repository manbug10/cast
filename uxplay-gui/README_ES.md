# UxPlay GUI

Una interfaz gráfica moderna para UxPlay construida con Rust y Tauri.

## Características

- 🎨 **Interfaz Moderna**: Diseño atractivo y fácil de usar
- 🚀 **Rendimiento**: Construido con Rust para máximo rendimiento
- 🔧 **Configuración Fácil**: Ajusta todos los parámetros de UxPlay desde la GUI
- 📊 **Logs en Tiempo Real**: Visualiza los registros de UxPlay mientras se ejecuta
- 💻 **Multiplataforma**: Funciona en Windows, macOS y Linux

## Capturas

La interfaz incluye:
- Campo para nombre del dispositivo
- Configuración de puerto personalizado
- Modo verbose para debugging
- Opción de solo audio
- Modo pantalla completa
- Botones de inicio/parada
- Panel de estado
- Consola de logs en tiempo real

## Requisitos Previos

### Para Windows

1. **UxPlay compilado** (ver [GUÍA DE INSTALACIÓN](WINDOWS_BUILD_GUIDE.md))
2. **Rust** - https://rustup.rs/
3. **Node.js** - https://nodejs.org/

### Para Linux

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4-devel openssl-devel curl wget gtk3-devel libappindicator-gtk3-devel librsvg2-devel
```

## Instalación

### 1. Clonar el repositorio

```bash
git clone https://github.com/tu-usuario/uxplay-gui.git
cd uxplay-gui
```

### 2. Instalar dependencias

```bash
npm install
```

### 3. Desarrollo

```bash
npm run tauri dev
```

### 4. Compilar para producción

```bash
npm run tauri build
```

Los ejecutables se encontrarán en `src-tauri/target/release/`

## Uso

1. **Iniciar UxPlay**:
   - Ingresa un nombre para tu dispositivo
   - Configura las opciones deseadas
   - Haz clic en "Start UxPlay"

2. **Conectar desde iOS/macOS**:
   - Abre el Centro de Control
   - Selecciona "Duplicar pantalla"
   - Elige tu dispositivo de la lista

3. **Monitorear**:
   - Los logs aparecen en tiempo real
   - El estado se muestra en la parte superior

## Estructura del Proyecto

```
uxplay-gui/
├── src/                    # Frontend (HTML/CSS/JS)
│   └── index.html
├── src-tauri/              # Backend (Rust)
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── README.md
```

## Comandos Disponibles

| Comando | Descripción |
|---------|-------------|
| `npm run tauri dev` | Inicia en modo desarrollo |
| `npm run tauri build` | Compila para producción |
| `npm run tauri` | Accede a la CLI de Tauri |

## Solución de Problemas

### "Failed to start uxplay"
- Verifica que UxPlay esté instalado y en el PATH
- Ejecuta `uxplay --version` en la terminal para verificar

### La aplicación no inicia
- Asegúrate de tener todas las dependencias del sistema instaladas
- Revisa los logs de error en la consola

### Problemas de audio/video
- Instala los plugins correctos de GStreamer
- Prueba activando/desactivando el modo "Solo audio"

## Tecnologías Utilizadas

- **Frontend**: HTML5, CSS3, JavaScript
- **Backend**: Rust
- **Framework**: Tauri v2
- **UI**: Diseño personalizado con CSS moderno

## Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature
3. Commit tus cambios
4. Push a la rama
5. Abre un Pull Request

## Licencia

Este proyecto está licenciado bajo la misma licencia que UxPlay.

## Enlaces Útiles

- [UxPlay Original](https://github.com/FDH2/UxPlay)
- [Documentación de Tauri](https://tauri.app/)
- [Guía de Instalación para Windows](WINDOWS_BUILD_GUIDE.md)

## Soporte

Para problemas específicos de UxPlay, consulta el repositorio original.
Para problemas de la GUI, abre un issue en este repositorio.
