use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use anyhow::{Context, Result};
use inquire::{Confirm, Password, Select, Text};
use regex::Regex;
use serde::{Deserialize, Serialize};
use clap::Parser;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Parser, Debug)]
#[command(name = "ssh-manager")]
#[command(about = "A tool to manage multiple SSH accounts easily")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Add a new SSH account
    Add,
    /// List all accounts
    List,
    /// Switch between accounts
    Switch,
    /// Remove an account
    Remove,
    /// Show current active account
    Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SshAccount {
    name: String,
    email: String,
    key_file: String,
    host: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    accounts: HashMap<String, SshAccount>,
    current_account: Option<String>,
}

struct SshManager {
    config_path: PathBuf,
    ssh_dir: PathBuf,
    config: Config,
}

impl SshManager {
    fn new() -> Result<Self> {
        let home_dir = home::home_dir().context("Could not find home directory")?;
        let ssh_dir = home_dir.join(".ssh");
        let config_path = ssh_dir.join("ssh_manager_config.json");
        
        // Ensure .ssh directory exists
        if !ssh_dir.exists() {
            fs::create_dir_all(&ssh_dir).context("Failed to create .ssh directory")?;
        }
        
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("Failed to read config file")?;
            serde_json::from_str(&content)
                .context("Failed to parse config file")?
        } else {
            Config::default()
        };
        
        Ok(SshManager {
            config_path,
            ssh_dir,
            config,
        })
    }
    
    fn save_config(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.config)
            .context("Failed to serialize config")?;
        fs::write(&self.config_path, json)
            .context("Failed to write config file")?;
        Ok(())
    }
    
    fn validate_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }
    
    fn add_account(&mut self) -> Result<()> {
        println!("\n🔑 Adding a new SSH account\n");
        
        let name = Text::new("Account name (e.g., 'work', 'personal', 'github-work'):")
            .prompt()
            .context("Failed to get account name")?;
        
        if self.config.accounts.contains_key(&name) {
            println!("❌ Account '{}' already exists!", name);
            return Ok(());
        }
        
        let email = loop {
            let input = Text::new("Email address:")
                .prompt()
                .context("Failed to get email")?;
            
            if Self::validate_email(&input) {
                break input;
            } else {
                println!("❌ Please enter a valid email address");
            }
        };
        
        let host = Select::new(
            "Select the host type:",
            vec!["github.com", "gitlab.com", "bitbucket.org", "Custom"],
        )
        .prompt()
        .context("Failed to get host selection")?;
        
        let host = if host == "Custom" {
            Text::new("Enter custom host:")
                .prompt()
                .context("Failed to get custom host")?
        } else {
            host.to_string()
        };
        
        let description = Text::new("Description (optional):")
            .with_default("")
            .prompt()
            .context("Failed to get description")?;
        
        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };
        
        // Ask for passphrase
        let use_passphrase = Confirm::new("Do you want to set a passphrase for this key?")
            .with_default(true)
            .prompt()
            .context("Failed to get passphrase confirmation")?;
        
        let passphrase = if use_passphrase {
            Some(Password::new("Enter passphrase for the SSH key:")
                .without_confirmation()
                .prompt()
                .context("Failed to get passphrase")?)
        } else {
            None
        };
        
        // Generate SSH key
        let key_file = format!("id_{}_{}", name.replace(" ", "_"), host.replace(".", "_"));
        let key_path = self.ssh_dir.join(&key_file);

        // Handle overwrite if key already exists
        if key_path.exists() || key_path.with_extension("pub").exists() {
            let overwrite = Confirm::new(&format!(
                "Key {} already exists. Overwrite?",
                key_path.display()
            ))
            .with_default(false)
            .prompt()
            .context("Failed to confirm overwrite")?;

            if !overwrite {
                println!("❌ Key generation cancelled.");
                return Ok(());
            }

            let _ = fs::remove_file(&key_path);
            let _ = fs::remove_file(key_path.with_extension("pub"));
        }

        println!("\n🔄 Generating SSH key...");

        let status = Command::new("ssh-keygen")
            .arg("-t")
            .arg("ed25519")
            .arg("-C")
            .arg(&email)
            .arg("-f")
            .arg(&key_path)
            .arg("-N")
            .arg(passphrase.as_deref().unwrap_or(""))
            .arg("-q")
            .status()
            .context("Failed to run ssh-keygen")?;

        if !status.success() {
            return Err(anyhow::anyhow!("ssh-keygen failed"));
        }

        // Ensure private key permissions are 600 on Unix systems
        #[cfg(unix)]
        {
            let perms = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&key_path, perms).context("Failed to set key permissions to 600")?;
        }
        
        println!("✅ SSH key generated successfully!");
        
        // Add to ssh-agent (and Keychain on macOS when applicable)
        println!("🔄 Adding key to ssh-agent...");
        let mut add_cmd = Command::new("ssh-add");
        if cfg!(target_os = "macos") && passphrase.is_some() {
            add_cmd.arg("--apple-use-keychain");
        }
        let add_status = add_cmd
            .arg(&key_path)
            .status()
            .context("Failed to add key to ssh-agent")?;
        if add_status.success() {
            if cfg!(target_os = "macos") && passphrase.is_some() {
                println!("✅ Key added to ssh-agent and keychain!");
            } else {
                println!("✅ Key added to ssh-agent!");
            }
        }
        
        // Create account
        let account = SshAccount {
            name: name.clone(),
            email,
            key_file,
            host: host.clone(),
            description,
        };
        
        self.config.accounts.insert(name.clone(), account);
        self.save_config().context("Failed to save configuration")?;
        
        // Show public key
        let pub_key_path = format!("{}.pub", key_path.display());
        if let Ok(pub_key) = fs::read_to_string(&pub_key_path) {
            println!("\n📋 Your public key (copy this to {}):", host);
            println!("{}", pub_key.trim());
            println!(
                "\n🔗 Suggested SSH alias: {} (use git@{}:org/repo.git)",
                Self::alias_for(self.config.accounts.get(&name).unwrap()),
                Self::alias_for(self.config.accounts.get(&name).unwrap())
            );
        }
        
        // Ask if they want to update SSH config
        let update_config = Confirm::new("Do you want to update your SSH config file?")
            .with_default(true)
            .prompt()
            .context("Failed to get SSH config confirmation")?;
        
        if update_config {
            self.update_ssh_config(&name)?;
        }
        
        println!("\n🎉 Account '{}' added successfully!", name);
        Ok(())
    }
    
    fn update_ssh_config(&self, account_name: &str) -> Result<()> {
        let account = self.config.accounts.get(account_name)
            .context("Account not found")?;
        
        let ssh_config_path = self.ssh_dir.join("config");
        let key_path = self.ssh_dir.join(&account.key_file);
        
        // Create a per-account alias to avoid conflicts for the same host
        let alias = Self::alias_for(account);
        let host_config = format!(
            "\n# {} - {}\nHost {}\n    HostName {}\n    User git\n    IdentityFile {}\n    AddKeysToAgent yes\n    UseKeychain yes\n    IdentitiesOnly yes\n",
            account.name,
            account.description.as_deref().unwrap_or(&account.email),
            alias,
            account.host,
            key_path.display()
        );
        
        let current_config = if ssh_config_path.exists() {
            fs::read_to_string(&ssh_config_path)
                .context("Failed to read SSH config")?
        } else {
            String::new()
        };
        
        // Check if this host is already configured
        let host_marker = format!("# {} - ", account.name);
        if current_config.contains(&host_marker) {
            println!("ℹ️  SSH config for '{}' already exists, skipping...", account.name);
            return Ok(());
        }
        
        let updated_config = current_config + &host_config;
        
        fs::write(&ssh_config_path, updated_config)
            .context("Failed to write SSH config")?;
        
        println!("✅ SSH config updated!");
        Ok(())
    }

    // Compute a unique alias per account, e.g., "github-work"
    fn alias_for(account: &SshAccount) -> String {
        let host_prefix = account
            .host
            .split('.')
            .next()
            .unwrap_or(&account.host)
            .to_string();
        let name_part = account.name.replace(' ', "-");
        format!("{}-{}", host_prefix, name_part)
    }

    // Ensure an "active" mapping for the given host to use the provided key
    fn upsert_active_mapping(&self, host: &str, key_path: &PathBuf) -> Result<()> {
        let ssh_config_path = self.ssh_dir.join("config");
        let mut current_config = if ssh_config_path.exists() {
            fs::read_to_string(&ssh_config_path).context("Failed to read SSH config")?
        } else {
            String::new()
        };

        let start_marker = format!("# ssh-manager ACTIVE START [{}]\n", host);
        let end_marker = format!("# ssh-manager ACTIVE END [{}]\n", host);

        let mut block = String::new();
        block.push_str(&start_marker);
        block.push_str(&format!(
            "Host {}\n    HostName {}\n    User git\n    IdentityFile {}\n    AddKeysToAgent yes\n    UseKeychain yes\n    IdentitiesOnly yes\n",
            host,
            host,
            key_path.display()
        ));
        block.push_str(&end_marker);

        if let Some(start_idx) = current_config.find(&start_marker) {
            if let Some(after_start) = current_config.get(start_idx + start_marker.len()..) {
                if let Some(end_rel_idx) = after_start.find(&end_marker) {
                    let end_idx = start_idx + start_marker.len() + end_rel_idx + end_marker.len();
                    let mut new_config = String::with_capacity(current_config.len() + block.len());
                    new_config.push_str(&current_config[..start_idx]);
                    new_config.push_str(&block);
                    new_config.push_str(&current_config[end_idx..]);
                    current_config = new_config;
                } else {
                    // Start found but no end; replace from start with block
                    let mut new_config = String::with_capacity(current_config.len() + block.len());
                    new_config.push_str(&current_config[..start_idx]);
                    new_config.push_str(&block);
                    current_config = new_config;
                }
            }
        } else {
            if !current_config.ends_with('\n') && !current_config.is_empty() {
                current_config.push('\n');
            }
            current_config.push_str(&block);
        }

        fs::write(&ssh_config_path, current_config).context("Failed to write SSH config")?;
        println!("✅ Active SSH mapping updated for {}", host);
        Ok(())
    }

    fn clear_active_mapping_for_host(&self, host: &str) -> Result<()> {
        let ssh_config_path = self.ssh_dir.join("config");
        if !ssh_config_path.exists() {
            return Ok(());
        }

        let mut current_config = fs::read_to_string(&ssh_config_path).context("Failed to read SSH config")?;
        let start_marker = format!("# ssh-manager ACTIVE START [{}]\n", host);
        let end_marker = format!("# ssh-manager ACTIVE END [{}]\n", host);

        if let Some(start_idx) = current_config.find(&start_marker) {
            if let Some(after_start) = current_config.get(start_idx + start_marker.len()..) {
                if let Some(end_rel_idx) = after_start.find(&end_marker) {
                    let end_idx = start_idx + start_marker.len() + end_rel_idx + end_marker.len();
                    let mut new_config = String::with_capacity(current_config.len());
                    new_config.push_str(&current_config[..start_idx]);
                    new_config.push_str(&current_config[end_idx..]);
                    current_config = new_config;
                    fs::write(&ssh_config_path, current_config).context("Failed to write SSH config")?;
                    println!("ℹ️  Active SSH mapping cleared for {}", host);
                }
            }
        }
        Ok(())
    }

    fn remove_from_ssh_agent(&self, key_path: &PathBuf) {
        let _ = Command::new("ssh-add").arg("-d").arg(key_path).status();
    }

    fn remove_ssh_config_for_account(&self, account: &SshAccount) -> Result<()> {
        let ssh_config_path = self.ssh_dir.join("config");
        if !ssh_config_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&ssh_config_path)
            .context("Failed to read SSH config")?;

        let alias = Self::alias_for(account);
        let name_escaped = regex::escape(&account.name);
        let alias_escaped = regex::escape(&alias);

        // Remove block that starts with our comment header and includes the alias Host block
        let header_pattern = format!(
            r"(?m)\n?# {}\s-\s[^\n]*\nHost {}\n(?:[ \t].*\n)*",
            name_escaped, alias_escaped
        );
        let re_header = Regex::new(&header_pattern).context("Failed to compile regex")?;
        let after_header = re_header.replace_all(&content, "");

        // Fallback: remove a Host-alias block without the header if present
        let host_pattern = format!(r"(?m)^\n?Host {}\n(?:[ \t].*\n)*", alias_escaped);
        let re_host = Regex::new(&host_pattern).context("Failed to compile regex")?;
        let new_content = re_host.replace_all(&after_header, "");

        if new_content != content {
            fs::write(&ssh_config_path, new_content.as_ref())
                .context("Failed to write SSH config")?;
            println!("✅ SSH config entry for '{}' removed.", alias);
        } else {
            println!("ℹ️  No SSH config entry found for '{}' (nothing to remove).", alias);
        }
        Ok(())
    }
    
    fn list_accounts(&self) -> Result<()> {
        if self.config.accounts.is_empty() {
            println!("📭 No accounts found. Use 'ssh-manager add' to create one.");
            return Ok(());
        }
        
        println!("\n📋 SSH Accounts:\n");
        
        for (name, account) in &self.config.accounts {
            let active = if Some(name) == self.config.current_account.as_ref() {
                "🟢 (active)"
            } else {
                "⚪"
            };
            
            println!("  {} {} ({})", active, name, account.email);
            println!("      Host: {}", account.host);
            if let Some(desc) = &account.description {
                println!("      Description: {}", desc);
            }
            println!();
        }
        
        Ok(())
    }
    
    fn switch_account(&mut self) -> Result<()> {
        if self.config.accounts.is_empty() {
            println!("📭 No accounts found. Use 'ssh-manager add' to create one.");
            return Ok(());
        }
        
        let account_names: Vec<String> = self.config.accounts.keys().cloned().collect();
        
        let selected = Select::new("Select account to activate:", account_names)
            .prompt()
            .context("Failed to get account selection")?;
        
        self.config.current_account = Some(selected.clone());
        self.save_config().context("Failed to save configuration")?;

        // Update active host mapping to point host -> selected account key
        if let Some(account) = self.config.accounts.get(&selected) {
            let key_path = self.ssh_dir.join(&account.key_file);
            self.upsert_active_mapping(&account.host, &key_path)?;
        }
        
        println!("✅ Switched to account '{}'", selected);
        Ok(())
    }
    
    fn show_status(&self) -> Result<()> {
        if let Some(current) = &self.config.current_account {
            if let Some(account) = self.config.accounts.get(current) {
                println!("\n🟢 Current active account: {} ({})", current, account.email);
                println!("   Host: {}", account.host);
                if let Some(desc) = &account.description {
                    println!("   Description: {}", desc);
                }
                
                // Test SSH connection
                println!("\n🔄 Testing SSH connection...");
                let key_path = self.ssh_dir.join(&account.key_file);
                let output = Command::new("ssh")
                    .arg("-T")
                    .arg("-i")
                    .arg(&key_path)
                    .arg(&format!("git@{}", account.host))
                    .output();
                
                match output {
                    Ok(result) => {
                        let stderr = String::from_utf8_lossy(&result.stderr);
                        if stderr.contains("successfully authenticated") {
                            println!("✅ SSH connection successful!");
                        } else if stderr.contains("Permission denied") {
                            println!("❌ SSH connection failed - key not added to {} or incorrect key", account.host);
                        } else {
                            println!("ℹ️  SSH test result: {}", stderr.trim());
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Could not test SSH connection: {}", e);
                    }
                }
            } else {
                println!("❌ Current account '{}' not found in configuration", current);
            }
        } else {
            println!("📭 No active account set. Use 'ssh-manager switch' to select one.");
        }
        
        Ok(())
    }
    
    fn remove_account(&mut self) -> Result<()> {
        if self.config.accounts.is_empty() {
            println!("📭 No accounts found.");
            return Ok(());
        }
        
        let account_names: Vec<String> = self.config.accounts.keys().cloned().collect();
        
        let selected = Select::new("Select account to remove:", account_names)
            .prompt()
            .context("Failed to get account selection")?;
        
        let confirm = Confirm::new(&format!("Are you sure you want to remove account '{}'?", selected))
            .with_default(false)
            .prompt()
            .context("Failed to get confirmation")?;
        
        if !confirm {
            println!("❌ Removal cancelled.");
            return Ok(());
        }
        
        if let Some(account) = self.config.accounts.remove(&selected) {
            // Remove key files if they exist
            let key_path = self.ssh_dir.join(&account.key_file);
            let pub_key_path = format!("{}.pub", key_path.display());
            
            let _ = fs::remove_file(&key_path);
            let _ = fs::remove_file(&pub_key_path);

            // Remove from ssh-agent if loaded
            self.remove_from_ssh_agent(&key_path);

            // Remove this account's alias block from ~/.ssh/config
            let _ = self.remove_ssh_config_for_account(&account);
            
            // Remove from current account if it was active
            if Some(&selected) == self.config.current_account.as_ref() {
                self.config.current_account = None;
                // Clear active mapping for this host
                let _ = self.clear_active_mapping_for_host(&account.host);
            }
            
            self.save_config().context("Failed to save configuration")?;
            
            println!("✅ Account '{}' removed successfully!", selected);
            println!("ℹ️  Note: SSH config entries need to be manually removed if desired.");
        }
        
        Ok(())
    }
    
    fn view_ssh_config(&self) -> Result<()> {
        let ssh_config_path = self.ssh_dir.join("config");
        println!("\n📄 SSH config path: {}\n", ssh_config_path.display());
        if !ssh_config_path.exists() {
            println!("📭 No SSH config file found.");
            return Ok(());
        }
        let content = fs::read_to_string(&ssh_config_path)
            .context("Failed to read SSH config")?;
        println!("──────── BEGIN ~/.ssh/config ────────");
        print!("{}", content);
        if !content.ends_with('\n') {
            println!();
        }
        println!("────────  END ~/.ssh/config  ────────");
        Ok(())
    }
    
    fn interactive_menu(&mut self) -> Result<()> {
        loop {
            let options = vec![
                "📝 Add new account",
                "📋 List accounts",
                "🔄 Switch account",
                "📊 Show status",
                "📄 View SSH config",
                "🗑️  Remove account",
                "🚪 Exit",
            ];
            
            let selection = Select::new("\n🔑 SSH Account Manager - What would you like to do?", options)
                .prompt()
                .context("Failed to get menu selection")?;
            
            match selection {
                "📝 Add new account" => self.add_account()?,
                "📋 List accounts" => self.list_accounts()?,
                "🔄 Switch account" => self.switch_account()?,
                "📊 Show status" => self.show_status()?,
                "📄 View SSH config" => self.view_ssh_config()?,
                "🗑️  Remove account" => self.remove_account()?,
                "🚪 Exit" => {
                    println!("👋 Goodbye!");
                    break;
                }
                _ => unreachable!(),
            }
            
            // Pause before showing menu again
            println!("\nPress Enter to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
        }
        
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut manager = SshManager::new()
        .context("Failed to initialize SSH manager")?;
    
    match args.command {
        Some(Commands::Add) => manager.add_account(),
        Some(Commands::List) => manager.list_accounts(),
        Some(Commands::Switch) => manager.switch_account(),
        Some(Commands::Remove) => manager.remove_account(),
        Some(Commands::Status) => manager.show_status(),
        None => manager.interactive_menu(),
    }
}
