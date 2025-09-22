# Git Account Manager (gam) 🔑

Una herramienta en Rust para gestionar múltiples cuentas SSH de Git de forma fácil e interactiva. Perfecta para desarrolladores que trabajan con múltiples cuentas de GitHub, GitLab, Bitbucket o servidores personalizados.

## ✨ Características

- 🔐 **Generación de claves ED25519** (no interactiva con `-N`), passphrase opcional
- 🛡️ **Permisos 600** en la clave privada e `IdentitiesOnly yes`
- 👥 **Gestión de múltiples cuentas** (trabajo, personal, etc.)
- 🧩 **Aliases SSH por cuenta** (p. ej., `Host github-work`)
- 🔄 **Cambio de cuenta con mapeo activo por host** (actualiza `Host github.com` → clave activa)
- ⚙️ **Actualización segura del SSH config** y limpieza del bloque de la cuenta al eliminarla
- 👁️ **Ver `~/.ssh/config`** desde el menú interactivo
- 🍎 **Integración con macOS Keychain**
- 🎨 **Interfaz interactiva con emojis**
- 🧪 **Prueba de conexión SSH**
- 📧 **Validación de emails**

## 🚀 Instalación

### Prerequisitos
- Rust (instalar desde [rustup.rs](https://rustup.rs/))
- macOS (por la integración con Keychain)

### Compilar e instalar

```bash
cd /Users/gio/Documents/ssh-manager
cargo build --release
sudo cp target/release/gam /usr/local/bin/
```

O para usar directamente:

```bash
cargo run --bin gam
```

O usando el script de instalación:

```bash
./install.sh
```

### Instalar desde Releases (multiplataforma)

En cada tag `vX.Y.Z` se generan binarios para Linux, macOS (Intel/Apple Silicon) y Windows.

- Descarga el archivo correspondiente desde “Releases” y añade el binario a tu `PATH`.

- Linux (x86_64):

```bash
VERSION="vX.Y.Z"
curl -L -o gam.tar.gz \
  "https://github.com/<TU_USUARIO>/<TU_REPO>/releases/download/${VERSION}/gam-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
tar -xzf gam.tar.gz
sudo install -m 0755 gam/gam /usr/local/bin/gam
```

- Linux (ARM64):

```bash
VERSION="vX.Y.Z"
curl -L -o gam.tar.gz \
  "https://github.com/<TU_USUARIO>/<TU_REPO>/releases/download/${VERSION}/gam-${VERSION}-aarch64-unknown-linux-gnu.tar.gz"
tar -xzf gam.tar.gz
sudo install -m 0755 gam/gam /usr/local/bin/gam
```

- macOS (Intel):

```bash
VERSION="vX.Y.Z"
curl -L -o gam.tgz \
  "https://github.com/<TU_USUARIO>/<TU_REPO>/releases/download/${VERSION}/gam-${VERSION}-x86_64-apple-darwin.tar.gz"
tar -xzf gam.tgz
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

Verificación de integridad (opcional):

```bash
shasum -a 256 -c gam-${VERSION}-SHA256SUMS.txt
```

## 📖 Uso

### Modo interactivo (recomendado)

```bash
gam
```

Esto iniciará un menú interactivo donde puedes:

- 📝 Agregar nueva cuenta
- 📋 Listar cuentas existentes
- 🔄 Cambiar entre cuentas
- 📊 Mostrar estado actual
- 📄 Ver SSH config
- 🗑️ Eliminar cuentas

### Comandos directos

```bash
# Agregar nueva cuenta
gam add

# Listar todas las cuentas
gam list

# Cambiar cuenta activa
gam switch

# Mostrar cuenta actual y probar conexión
gam status

# Eliminar una cuenta
gam remove
```

## 📁 Estructura de archivos

Git Account Manager crea y gestiona los siguientes archivos:

```
~/.ssh/
├── gam_config.json             # Configuración del manager (compatible con ssh_manager_config.json)
├── config                      # SSH config (actualizado automáticamente)
├── id_accountname_hostname     # Claves privadas
└── id_accountname_hostname.pub # Claves públicas
```

## 🧩 Aliases y remotos Git

Al agregar una cuenta se puede escribir un bloque por cuenta en `~/.ssh/config` con un **alias único** para evitar conflictos cuando hay varias cuentas en el mismo host.

- Ejemplo de alias para cuenta `work` en `github.com`: `Host github-work` con `HostName github.com` y su `IdentityFile`.
- Para usar el alias en Git:

```bash
git remote set-url origin git@github-work:org/repo.git
```

Además, al usar `gam switch`, se actualiza un bloque “activo” para el host real (p. ej., `Host github.com`) que apunta a la clave de la cuenta seleccionada. Así, si usas `git@github.com:org/repo.git` sin alias, se usará la cuenta activa.

Recomendación: utiliza aliases para remotos de Git si gestionas varias identidades en el mismo host, y usa `switch` para cambiar rápidamente la identidad por defecto del host.

## 🔧 Ejemplo de uso

### 1. Agregar cuenta de trabajo

```bash
gam add
```

```
🔑 Adding a new SSH account

? Account name: work
? Email address: juan@empresa.com
? Select the host type: github.com
? Description: Cuenta de trabajo GitHub
? Do you want to set a passphrase: Yes
? Enter passphrase: [hidden]

🔄 Generating SSH key...
✅ SSH key generated successfully!
🔄 Adding key to ssh-agent and keychain...
✅ Key added to ssh-agent and keychain!

📋 Your public key (copy this to github.com):
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA... juan@empresa.com

? Do you want to update your SSH config file: Yes
✅ SSH config updated!

🎉 Account 'work' added successfully!
```

### 2. Agregar cuenta personal

```bash
gam add
```

Similar al anterior, pero con datos personales.

### 3. Cambiar entre cuentas

```bash
gam switch
```

```
? Select account to activate:
  work
❯ personal

✅ Switched to account 'personal'
```

### 4. Ver estado actual

```bash
gam status
```

```
🟢 Current active account: personal (juan.personal@gmail.com)
   Host: github.com
   Description: Cuenta personal GitHub

🔄 Testing SSH connection...
✅ SSH connection successful!
```

## 🔒 Seguridad

- **Claves ED25519**: Usa el algoritmo más seguro y moderno
- **Passphrases**: Opción de proteger las claves con passphrase
- **Keychain**: Integración segura con macOS Keychain
- **Permisos**: Mantiene los permisos correctos en archivos SSH

## ❓ Problemas comunes

### "Permission denied" en conexión SSH

1. Asegúrate de haber copiado la clave pública a tu servicio (GitHub, GitLab, etc.)
2. Verifica que la clave esté agregada al ssh-agent: `ssh-add -l`
3. Prueba la conexión manualmente: `ssh -T git@github.com`

### Error de compilación

Asegúrate de tener Rust instalado y actualizado:

```bash
rustup update
cargo build --release
```

## 🤝 Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature
3. Commit tus cambios
4. Push a la rama
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver el archivo LICENSE para más detalles.

## 🙏 Agradecimientos

- [inquire](https://crates.io/crates/inquire) - Por la interfaz interactiva
- [clap](https://crates.io/crates/clap) - Por el parsing de argumentos
- [serde](https://crates.io/crates/serde) - Por la serialización JSON