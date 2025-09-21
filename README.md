# SSH Manager 🔑

Una herramienta en Rust para gestionar múltiples cuentas SSH de forma fácil e interactiva. Perfecta para desarrolladores que trabajan con múltiples cuentas de GitHub, GitLab, Bitbucket o servidores personalizados.

## ✨ Características

- 🔐 **Generación automática de claves SSH** con algoritmo ED25519
- 👥 **Gestión de múltiples cuentas** (trabajo, personal, etc.)
- 🔄 **Cambio fácil entre cuentas**
- 📧 **Validación de emails**
- 🔒 **Soporte para passphrases**
- 🍎 **Integración con macOS Keychain**
- ⚙️ **Actualización automática del SSH config**
- 🎨 **Interfaz interactiva con emojis**
- 🧪 **Prueba de conexión SSH**

## 🚀 Instalación

### Prerequisitos
- Rust (instalar desde [rustup.rs](https://rustup.rs/))
- macOS (por la integración con Keychain)

### Compilar e instalar

```bash
cd /Users/gio/Documents/ssh-manager
cargo build --release
sudo cp target/release/ssh-manager /usr/local/bin/
```

O para usar directamente:

```bash
cargo run
```

## 📖 Uso

### Modo interactivo (recomendado)

```bash
ssh-manager
```

Esto iniciará un menú interactivo donde puedes:

- 📝 Agregar nueva cuenta
- 📋 Listar cuentas existentes
- 🔄 Cambiar entre cuentas
- 📊 Mostrar estado actual
- 🗑️ Eliminar cuentas

### Comandos directos

```bash
# Agregar nueva cuenta
ssh-manager add

# Listar todas las cuentas
ssh-manager list

# Cambiar cuenta activa
ssh-manager switch

# Mostrar cuenta actual y probar conexión
ssh-manager status

# Eliminar una cuenta
ssh-manager remove
```

## 📁 Estructura de archivos

El SSH Manager crea y gestiona los siguientes archivos:

```
~/.ssh/
├── ssh_manager_config.json     # Configuración del manager
├── config                      # SSH config (actualizado automáticamente)
├── id_accountname_hostname     # Claves privadas
└── id_accountname_hostname.pub # Claves públicas
```

## 🔧 Ejemplo de uso

### 1. Agregar cuenta de trabajo

```bash
ssh-manager add
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
ssh-manager add
```

Similar al anterior, pero con datos personales.

### 3. Cambiar entre cuentas

```bash
ssh-manager switch
```

```
? Select account to activate:
  work
❯ personal

✅ Switched to account 'personal'
```

### 4. Ver estado actual

```bash
ssh-manager status
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