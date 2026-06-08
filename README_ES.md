# 📡 UxPlay GUI - Windows (Todo en Uno)

Interfaz gráfica moderna para **UxPlay** construida con **Rust + Tauri**. Este proyecto incluye un workflow de GitHub Actions que compila automáticamente tanto UxPlay como la GUI en un único ejecutable portable para Windows.

## ✨ Características

- **Un solo archivo**: Todo incluido (GUI + Motor UxPlay)
- **Sin dependencias externas**: No necesitas instalar MSYS2, GStreamer ni nada más en tu PC
- **Interfaz moderna**: Diseño oscuro con controles intuitivos
- **Configuración completa**: Nombre, puerto, modo verbose, solo audio, fullscreen
- **Logs en tiempo real**: Visualiza la salida del servidor directamente en la app
- **Portable**: Ejecuta y listo

## 🚀 Cómo obtener el ejecutable

### Opción A: Descargar desde Releases (Recomendado)

1. Ve a la pestaña **Releases** de este repositorio
2. Descarga el archivo `uxplay-gui.exe` de la última versión
3. Ejecútalo directamente

### Opción B: Compilar tú mismo con GitHub Actions

1. Haz un fork de este repositorio
2. Ve a la pestaña **Actions**
3. Selecciona el workflow **"Build UxPlay GUI for Windows"**
4. Haz clic en **"Run workflow"**
5. Espera a que termine (aprox. 10-15 minutos)
6. Descarga el artefacto generado

### Opción C: Compilar localmente (Solo si tienes Windows)

```bash
# 1. Instalar Rust desde https://rustup.rs/
# 2. Instalar Node.js desde https://nodejs.org/
# 3. Instalar MSYS2 y dependencias (ver guía detallada abajo)

# Compilar UxPlay primero en MSYS2 UCRT64
git clone https://github.com/FDH2/UxPlay.git
cd UxPlay/build
cmake -G Ninja -DCMAKE_BUILD_TYPE=Release ..
ninja
cp uxplay.exe ../../src-tauri/uxplay_windows.exe

# Volver al proyecto y compilar la GUI
cd ../../
npm install
npm run tauri build
```

## 🎮 Uso

1. **Ejecuta** `uxplay-gui.exe`
2. **Configura** el nombre del dispositivo (por defecto: "UxPlay-PC")
3. **Opcional**: Ajusta puerto, modo verbose, etc.
4. **Haz clic** en "▶ Iniciar Servidor"
5. **Conéctate** desde tu iPhone/iPad/Mac usando AirPlay
6. **Detén** cuando termines con "⏹ Detener"

## 🛠️ Tecnologías

- **Frontend**: HTML5, CSS3, JavaScript vanilla
- **Backend**: Rust
- **Framework GUI**: Tauri v1
- **Motor AirPlay**: UxPlay (compilado estáticamente en el exe)

## 📝 Notas Importantes

- **Firewall**: La primera vez que lo ejecutes, Windows Defender puede pedirte permiso para aceptar conexiones entrantes. Debes **aceptar** para que funcione AirPlay.
- **Red**: Tu PC y dispositivo Apple deben estar en la misma red Wi-Fi.
- **Codecs**: Esta versión incluye los codecs básicos. Para formatos especiales, podrías necesitar codecs adicionales en Windows.

## 🐛 Solución de Problemas

| Problema | Solución |
|----------|----------|
| No aparece en AirPlay | Verifica firewall y que estén en la misma red |
| Error al iniciar | Revisa que no haya otro UxPlay ejecutándose |
| Sin audio/video | Instala codecs K-Lite o similar en Windows |
| Se cierra inesperadamente | Ejecuta como administrador una vez |

## 📄 Licencia

MIT License - El código de UxPlay original tiene su propia licencia (GPL).

## 🙏 Créditos

- **UxPlay**: [FDH2/UxPlay](https://github.com/FDH2/UxPlay)
- **Tauri**: [tauri.app](https://tauri.app/)

---

¡Disfruta de AirPlay en tu Windows! 🍏➡️🪟
