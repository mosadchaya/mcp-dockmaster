# MCP Dockmaster Architecture

This document outlines the architecture of the MCP Dockmaster core library, which follows a layered architecture pattern with clear separation of concerns.

## Architectural Overview

The codebase is organized into four main layers:

```
src/
├── api/               # Interface Layer - HTTP/RPC endpoints
├── application/       # Application Layer - Use cases and orchestration
├── domain/            # Domain Layer - Core business logic and entities
├── infrastructure/    # Infrastructure Layer - External systems integration
├── models/            # Legacy models (being migrated to domain)
└── lib.rs             # Main entry point
```

## Layer Responsibilities

### Domain Layer

The Domain Layer contains the core business logic, entities, and rules of the application. It is independent of other layers and frameworks.

```
domain/
├── entities/          # Business objects (Tool, etc.)
├── value_objects/     # Immutable value types (ToolId, etc.)
├── errors.rs          # Domain-specific errors
├── services.rs        # Core business logic
└── traits.rs          # Interfaces for repositories and services
```

**Key Components:**
- **Entities**: Core business objects like `Tool`
- **Value Objects**: Immutable types like `ToolId`
- **Domain Services**: Core business logic
- **Repository Interfaces**: Abstractions for data access

### Application Layer

The Application Layer orchestrates the flow of data and coordinates domain objects to perform specific use cases.

```
application/
├── dto/               # Data Transfer Objects
└── services/          # Application services
```

**Key Components:**
- **DTOs**: Data Transfer Objects for API communication
- **Application Services**: Orchestrate domain objects to fulfill use cases
- **Command/Query Handlers**: Process specific commands or queries

### Infrastructure Layer

The Infrastructure Layer provides implementations for interfaces defined in the domain layer and handles external concerns.

```
infrastructure/
├── persistence/       # Database implementations
├── process_management/# Process handling implementations
└── repository.rs      # Repository implementations
```

**Key Components:**
- **Repository Implementations**: Concrete implementations of domain repositories
- **Process Management**: Handles process spawning and communication
- **External Services**: Integration with external systems

### API Layer

The API Layer handles HTTP requests, RPC calls, and other external interfaces.

```
api/
├── handlers/          # Request handlers
├── rpc/               # RPC models and protocols
├── routes.rs          # Route definitions
└── server.rs          # HTTP server setup
```

**Key Components:**
- **Handlers**: Process incoming requests
- **Routes**: Define API endpoints
- **RPC Models**: Request/response structures
- **Server**: HTTP server configuration

## Flow of Control

1. **Request Flow**:
   - API Layer receives requests
   - Application Layer orchestrates the use case
   - Domain Layer executes business logic
   - Infrastructure Layer handles persistence and external systems

2. **Dependency Direction**:
   - Dependencies flow inward (API → Application → Domain)
   - Domain layer has no dependencies on outer layers
   - Infrastructure implements interfaces defined in the domain

## Making Changes

When adding new features or modifying existing ones:

1. **Domain Changes**:
   - Add/modify entities in `domain/entities/`
   - Define new value objects in `domain/value_objects/`
   - Add domain-specific errors to `domain/errors.rs`
   - Define repository interfaces in `domain/traits.rs`

2. **Application Changes**:
   - Add DTOs in `application/dto/`
   - Create/modify services in `application/services/`

3. **Infrastructure Changes**:
   - Implement repository interfaces in `infrastructure/persistence/`
   - Add process management in `infrastructure/process_management/`

4. **API Changes**:
   - Add/modify handlers in `api/handlers/`
   - Update routes in `api/routes.rs`
   - Define RPC models in `api/rpc/`

## Testing

- **Unit Tests**: Test domain logic in isolation
- **Integration Tests**: Test across layers
- **Mock Implementations**: Use mock implementations for testing (e.g., `MockProcessManager`)

## Example: Adding a New Tool Feature

1. Define the tool entity in `domain/entities/tool.rs`
2. Create repository interface in `domain/traits.rs`
3. Implement business logic in `domain/services.rs`
4. Create DTOs in `application/dto/tool_dto.rs`
5. Implement application service in `application/services/tool_service.rs`
6. Add repository implementation in `infrastructure/repository.rs`
7. Create API handler in `api/handlers/tools_handler.rs`
8. Update routes in `api/routes.rs`
