# 🚨 AlertManager Discord Bot

> Transform your Kubernetes alerts into organized Discord notifications with style!

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Kubernetes](https://img.shields.io/badge/kubernetes-ready-brightgreen.svg)
![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)

<p align="center">
  <img src="docs/assets/preview.png" alt="Bot Preview" width="600">
</p>

## 🎯 What is this?

This bot bridges the gap between AlertManager and Discord, making your Kubernetes alerts accessible and actionable right in your team's Discord channels!

### Why use it?

- 🔄 **Real-time Alerts**: Get your AlertManager notifications instantly in Discord
- 📊 **Organized Views**: Use Discord forums (or threads) for categorized alert management
- 🎨 **Rich Formatting**: Beautifully formatted messages with alert details
- 🚀 **Dead Simple Setup**: Just set a channel ID and you're good to go!

## 🏃‍♂️ Quick Start

1. **Invite the bot** to your Discord server
2. **Copy your channel ID** where you want the alerts
3. **Deploy** using our Helm chart (SOON™):

```bash
helm repo add alertmanager-discord https://your-repo.com/charts
helm install alertmanager-discord \
  --set discord.channelId=YOUR_CHANNEL_ID \
  --set discord.token=YOUR_BOT_TOKEN
```

Or use Docker:

```bash
docker run -e CHANNEL_ID=YOUR_CHANNEL_ID \
          -e DISCORD_TOKEN=YOUR_BOT_TOKEN \
          ghcr.io/your-org/alertmanager-discord
```

> 📝 Note: Remember to replace `YOUR_CHANNEL_ID` and `YOUR_BOT_TOKEN` with actual values when deploying.

## 🔧 Configuration

Configure AlertManager to send webhooks to the bot:

```yaml
receivers:
- name: 'discord'
  webhook_configs:
    - url: 'http://alertmanager-discord:8080/webhook'
```

## 🌟 Features

- **Forum Support**: Organize alerts by category in Discord forums or threads in a simple channel
- **Smart Deduplication**: Merges related alerts intelligently
- **Rich Formatting**: Converts AlertManager data into readable Discord messages
- **Kubernetes Native**: Built to run in your K8s cluster

## 🤝 Contributing

We love contributions! Whether you're fixing bugs, improving documentation, or adding new features, your help is welcome.

### Areas we'd love help with:

- 📝 Documentation improvements
- 🎨 Message formatting enhancements
- 🔧 Additional alert processing features
- ✨ New notification interactions

Check our [Contributing Guide](CONTRIBUTING.md) to get started!

## 🎬 Getting Started with Development

```bash
# Clone the repository
git clone https://github.com/shiipou/alertmanager-discord-rs

# Install dependencies
cargo build

# Run tests
cargo test

# Start local development
cargo run
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💖 Support

If this project helps you, please consider giving it a star ⭐️

---

<p align="center">
Made with ❤️ by the a DevOps, for the Community
</p>
