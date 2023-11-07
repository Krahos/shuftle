# Shuftle

A Rust-based deck game project.

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Setup](#setup)
  - [Prerequisites](#prerequisites)
  - [Running the Project](#running-the-project)
- [Contributing](#contributing)

## Overview

This open-source project is a deck game built in Rust, utilizing various components and services to create an interactive and scalable game. The project's main components include:

- Frontend Client
- API Gateway
- Keycloak for authentication (with an additional Postgres Database)
- Application Server
- Game Server
- Postgres Database

The project uses Docker Compose for containerization and gRPC for client-server communication.

## Project Structure

The project is divided into several layers and components, each handled by a different team member. Here's an overview of the project structure:

1. **Frontend Client**: The client application used to play the game;
2. **API Gateway**: Responsible for routing and handling incoming requests;
3. **Application Server**: Manages game logic and business rules;
4. **Game Server**: Handles game sessions and real-time interactions;
5. **Postgres Database**: Stores necessary data for the game;
6. **Keycloak**: Provides authentication services for the project.

## Setup

### Prerequisites

Before you can run the project, ensure you have the following prerequisites installed on your system:

- [Docker](https://www.docker.com/get-started)
- [Rust](https://www.rust-lang.org/tools/install)

### Running the Project

To run the project, follow these steps:

1. Clone the repository:

```bash
   git clone https://github.com/Krahos/shuftle
```

2. Navigate to the project directory:

```bash
cd shuftle
```

3. Start the Docker containers using Docker Compose:

```bash
docker-compose up -d
```

This command will start all the project's containers, including Keycloak and the PostgreSQL database.

## Contributing

We welcome contributions to this open-source project. If you'd like to contribute, please follow these steps:

1. Fork the repository;

2. Create a feature branch with a descriptive name:

```bash
git checkout -b feature/your-feature-name
```

3. Make your changes and commit them:

```bash
git commit -m "Add your feature description"
```

4. Push your changes to your forked repository:

```bash
git push origin feature/your-feature-name
```

5. Open a Pull Request (PR) to the main repository;
6. Your PR will be reviewed and merged if approved.
