# Git Account Manager (gam)

### Qué es Git Account Manager
- **Gestiona múltiples identidades SSH de Git** (trabajo, personal, etc.) sin fricción.
- **Aliases por cuenta** (p. ej. `Host github-work`) para evitar conflictos en el mismo host.
- **Cambio rápido de identidad activa** por host (actualiza `~/.ssh/config` de forma segura).
- **Generación de claves ED25519** con passphrase opcional e integración con macOS Keychain.


### Funcionalidades
- Generar claves SSH (ED25519) con passphrase opcional e instalación en ssh-agent/Keychain.

```bash
gam add
```

- Gestionar cuentas: agregar, listar, cambiar activa, eliminar.

```bash
gam add
gam list
gam switch
gam remove
```

- Aliases por cuenta: crea `Host <alias>` con `HostName`, `IdentityFile` e `IdentitiesOnly yes`.
  - Usa el alias en tus remotos de Git para separar identidades por host.

```bash
git remote set-url origin git@github-work:org/repo.git
```

- Cambio de cuenta: actualiza un bloque activo `Host <host>` para usar la clave de la cuenta seleccionada.

```bash
gam switch
```

- Ver configuración: muestra el contenido de `~/.ssh/config` desde el menú.

```bash
gam
```

(En el menú, elige "📄 View SSH config")

- Limpieza segura: al eliminar una cuenta, quita solo el bloque de esa cuenta en `~/.ssh/config`.

```bash
gam remove
```

- Validaciones y seguridad: email válido, permisos 600 en clave privada y manejo de overwrite de claves.
- Compatibilidad macOS: añade la clave con `--apple-use-keychain` si aplica.

### Cómo instalarlo
- Requisitos solo si compilas desde fuente: Rust estable (`rustup.rs`).

- Opción A: Instalar binario desde Releases
  - Ve a la página de “Releases” de tu repositorio y descarga el artefacto para tu sistema (Linux/macOS/Windows).
  - Linux (x86_64):
```bash
VERSION="vX.Y.Z"
curl -L -o gam.tar.gz \
  "https://github.com/<TU_USUARIO>/<TU_REPO>/releases/download/${VERSION}/gam-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
tar -xzf gam.tar.gz
sudo install -m 0755 gam/gam /usr/local/bin/gam
```
  - macOS (Apple Silicon):
```bash
VERSION="vX.Y.Z"
curl -L -o gam.tgz \
  "https://github.com/<TU_USUARIO>/<TU_REPO>/releases/download/${VERSION}/gam-${VERSION}-aarch64-apple-darwin.tar.gz"
tar -xzf gam.tgz
sudo install -m 0755 gam/gam /usr/local/bin/gam
```
  - Windows (PowerShell):
```powershell
$Version = "vX.Y.Z"
Invoke-WebRequest -Uri "https://github.com/<TU_USUARIO>/<TU_REPO>/releases/download/$Version/gam-$Version-x86_64-pc-windows-msvc.zip" -OutFile gam.zip
Expand-Archive gam.zip -DestinationPath .
Move-Item -Force .\gam\gam.exe $Env:UserProfile\bin\gam.exe
# Asegúrate de tener %UserProfile%\bin en el PATH
```

- Opción B: Compilar desde fuente
```bash
# En macOS/Linux
cd /Users/gio/Documents/ssh-manager
cargo build --release
sudo cp target/release/gam /usr/local/bin/gam
# o bien
./install.sh
```

### Cómo contribuir
1) Haz un fork del repositorio
2) Crea una rama descriptiva: `git checkout -b feat/mi-cambio`
3) Desarrolla y valida localmente:
```bash
cargo build --release
cargo run --bin gam
```
4) Abre un Pull Request con una descripción clara

Notas:
- El CI ejecuta builds en Linux, macOS y Windows.
- Los binarios publicados en Releases se generan automáticamente al crear un tag `vX.Y.Z`.
****