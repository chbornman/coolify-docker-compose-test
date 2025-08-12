# Coolify Docker Compose Test Setup

This is a simple test setup for deploying a static website using Docker Compose on Coolify, based on the tutorial from [dev.to](https://dev.to/mandrasch/simple-coolify-example-with-docker-compose-github-deployments-53m).

## Project Structure

```
coolify_test/
├── web/
│   └── index.html          # Static HTML page
├── Dockerfile.prod         # Production Dockerfile using nginx:alpine
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

## Files Explanation

### `Dockerfile.prod`
Uses nginx:alpine to serve static files from the `/usr/share/nginx/html` directory.

### `docker-compose.production.yaml`
Minimal compose file for Coolify - builds the Docker image but doesn't specify ports (Coolify handles this).

### `docker-compose.yaml`
Local development version with:
- Port mapping (8080:80)
- Volume mount for live editing of HTML files

### `web/index.html`
Simple HTML page with gradient background and modern styling to verify the deployment works.

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

## Resources

- [Original Tutorial](https://dev.to/mandrasch/simple-coolify-example-with-docker-compose-github-deployments-53m)
- [Coolify Documentation](https://coolify.io/docs)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [nginx Docker Image](https://hub.docker.com/_/nginx)