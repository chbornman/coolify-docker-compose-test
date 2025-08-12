# Coolify Docker Compose Test Setup with Counter App

A full-stack counter application demonstrating Docker Compose deployment on Coolify, featuring a Rust Axum backend with SQLite database.

## Features

- 🦀 **Rust Axum Backend** - Fast, type-safe API server
- 📦 **SQLite Database** - Persistent counter storage
- 🎨 **Modern UI** - Interactive counter with increment/decrement/reset
- 🐳 **Docker Compose** - Multi-container orchestration
- 🚀 **Coolify Ready** - Optimized for Coolify deployment

## Project Structure

```
coolify_test/
├── backend/
│   ├── src/
│   │   └── main.rs         # Rust Axum API server
│   ├── Cargo.toml          # Rust dependencies
│   └── Dockerfile          # Backend container
├── web/
│   └── index.html          # Interactive counter UI
├── nginx.conf              # Nginx reverse proxy config
├── Dockerfile.prod         # Frontend nginx container
├── docker-compose.production.yaml  # For Coolify deployment
├── docker-compose.yaml     # For local testing
└── README.md
```

## Local Testing

To test this setup locally:

```bash
# Build and run the container
docker-compose up --build

# Visit http://localhost:8080 in your browser
```

To stop the container:
```bash
docker-compose down
```

## Deploying to Coolify

### Prerequisites
- A VPS with Coolify installed (e.g., Hetzner Cloud CPX11)
- GitHub account (for repository deployment)

### Deployment Steps

1. **Push this code to a GitHub repository**

2. **In Coolify:**
   - Create a new Project
   - Select "Production" environment
   - Add a new Resource
   - Choose "Public Repository" (Git Based)
   - Enter your GitHub repository URL
   - Switch Build Pack to "Docker Compose"
   - Set Compose Path to `/docker-compose.production.yaml`

3. **Configure Domain:**
   - Generate a temporary domain in Coolify
   - Or configure your own domain

4. **Deploy:**
   - Click the "Deploy" button
   - Wait for status to change to "Running"

### Important Notes for Coolify

- **DO NOT** specify ports in `docker-compose.production.yaml` - Coolify handles port mapping automatically
- If you need to mount files/directories, enable "Preserve Repository During Deployment" in Coolify settings
- The production compose file uses the build context, while local testing can use volume mounts for development

## API Endpoints

The backend provides these REST endpoints:

- `GET /api/counter` - Get current counter value
- `POST /api/counter/increment` - Increment counter by 1
- `POST /api/counter/decrement` - Decrement counter by 1
- `POST /api/counter/reset` - Reset counter to 0
- `GET /api/health` - Health check endpoint

## Files Explanation

### Backend Components
- **`backend/src/main.rs`** - Axum web server with SQLite integration
- **`backend/Dockerfile`** - Multi-stage build for optimized Rust binary
- **`backend/Cargo.toml`** - Rust dependencies (Axum, SQLx, Tokio)

### Frontend Components
- **`web/index.html`** - Interactive counter UI with JavaScript
- **`nginx.conf`** - Reverse proxy configuration for API routing
- **`Dockerfile.prod`** - Nginx container serving static files

### Docker Compose Files
- **`docker-compose.production.yaml`** - Production config without ports (Coolify manages this)
- **`docker-compose.yaml`** - Local development with port mappings and volumes

## Troubleshooting

### Out of Memory Issues
If you encounter "502 Bad Gateway" errors, consider adding swap space to your VPS:
```bash
# Check current swap
free -h

# Add swap if needed (follow your VPS provider's guide)
```

### Port Already Allocated Error
This happens if you specify ports in the production compose file. Remove any `ports:` configuration from `docker-compose.production.yaml`.

## Next Steps

This basic setup can be extended to:
- Add multiple services (database, Redis, etc.)
- Deploy dynamic applications (PHP, Node.js, Python)
- Configure environment variables
- Set up SSL certificates
- Add health checks

## Tech Stack

- **Backend**: Rust, Axum, SQLx, Tokio
- **Database**: SQLite (persistent volume)
- **Frontend**: HTML5, JavaScript, CSS3
- **Web Server**: Nginx (reverse proxy)
- **Containerization**: Docker, Docker Compose
- **Deployment**: Coolify

## Resources

- [Original Tutorial](https://dev.to/mandrasch/simple-coolify-example-with-docker-compose-github-deployments-53m)
- [Coolify Documentation](https://coolify.io/docs)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Axum Web Framework](https://github.com/tokio-rs/axum)
- [SQLx Database Library](https://github.com/launchbadge/sqlx)