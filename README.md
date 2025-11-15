# Quiz Backend API

A powerful Rust-based backend for generating and managing interactive quizzes. This project integrates with Strapi for headless CMS capabilities and provides GraphQL support for flexible querying.

## Overview

The Quiz Backend API is built with Rust and Actix-web, offering high-performance REST and GraphQL endpoints for:
- Quiz management and creation
- Question management with multiple question types
- Answer validation and scoring
- User quiz sessions and progress tracking
- Integration with Strapi CMS for content management

### Key Features

- **GraphQL & REST APIs**: Flexible API options for quiz data retrieval and management
- **Strapi Integration**: Seamless headless CMS integration for quiz content
- **High Performance**: Built with Rust for maximum reliability and speed
- **Scalable Architecture**: Modular design for easy feature expansion
- **Database Agnostic**: Sea-ORM abstraction layer for multiple database support
- **JWT Authentication**: Secure API access with role-based authorization
- **Docker Ready**: Containerized deployment out of the box

### Technology Stack

- **Backend**: Rust with Actix-web framework
- **GraphQL**: Juniper for GraphQL schema and execution
- **Database**: PostgreSQL with Sea-ORM
- **Authentication**: JWT-based authentication
- **CMS Integration**: Strapi headless CMS
- **Containerization**: Docker support
- **Deployment**: Fly.io configuration included

## Setup and Installation

### Prerequisites

- Rust (1.70.0 or later)
- PostgreSQL (14.0 or later)
- Strapi instance (for CMS integration)
- Docker (optional)

### Local Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/NsdHSO/quiz-backend.git
   cd quiz-backend
   ```

2. **Set up environment variables**
   Create a `.env` file in the project root:
   ```
   DATABASE_URL=postgres://username:password@localhost:5432/quiz_db
   HOST=127.0.0.1
   PORT=8080
   JWT_SECRET=your_jwt_secret_key
   STRAPI_URL=http://localhost:1337
   STRAPI_API_TOKEN=your_strapi_api_token
   ```

3. **Set up the database**
   ```bash
   createdb quiz_db
   cargo run --bin migration
   ```

4. **Run the application**
   ```bash
   cargo run
   ```

5. **Access the application**
   - REST API: http://localhost:8080/api
   - GraphQL: http://localhost:8080/graphql
   - GraphQL Playground: http://localhost:8080/graphql (in development)

### Docker Deployment

1. **Build the Docker image**
   ```bash
   docker build -t quiz-backend .
   ```

2. **Run the container**
   ```bash
   docker run -p 8080:8080 --env-file .env quiz-backend
   ```

## API Endpoints

### REST API

#### Quiz Management
- `GET /api/v1/quizzes` - List all quizzes
- `GET /api/v1/quizzes/{id}` - Get quiz details
- `POST /api/v1/quizzes` - Create a new quiz
- `PUT /api/v1/quizzes/{id}` - Update quiz information
- `DELETE /api/v1/quizzes/{id}` - Delete a quiz

#### Questions
- `GET /api/v1/quizzes/{quiz_id}/questions` - List questions in a quiz
- `GET /api/v1/questions/{id}` - Get question details
- `POST /api/v1/questions` - Create a new question
- `PUT /api/v1/questions/{id}` - Update a question
- `DELETE /api/v1/questions/{id}` - Delete a question

#### Quiz Sessions
- `POST /api/v1/sessions` - Start a new quiz session
- `GET /api/v1/sessions/{id}` - Get session details
- `POST /api/v1/sessions/{id}/answers` - Submit answer to a question
- `GET /api/v1/sessions/{id}/results` - Get session results

#### Authentication
- `POST /api/v1/auth/register` - Register a new user
- `POST /api/v1/auth/login` - Authenticate and get JWT token
- `GET /api/v1/auth/profile` - Get current user profile

### GraphQL API

Access the GraphQL playground at `/graphql` for interactive queries. Example queries:

```graphql
# Get all quizzes
query {
  quizzes {
    id
    title
    description
    questionCount
  }
}

# Get quiz with questions
query {
  quiz(id: "1") {
    id
    title
    questions {
      id
      text
      type
    }
  }
}

# Submit quiz answers
mutation {
  submitQuizAnswers(sessionId: "session-123", answers: [
    { questionId: "1", answer: "A" }
  ]) {
    score
    totalQuestions
    percentage
  }
}
```

## Strapi Integration

### Configuration

1. **Set up Strapi Content Types** for:
   - Quiz collections
   - Question collections
   - Answer options

2. **API Integration**
   - The backend automatically syncs quiz content from Strapi
   - Webhooks can be configured for real-time content updates

3. **Authentication**
   - Use Strapi API tokens for secure backend-to-CMS communication

### Example Strapi Setup

Create the following content types in Strapi:

**Quiz**
- Title (Text)
- Description (Rich Text)
- Category (Text)
- Difficulty (Enumeration)
- Questions (Relation to Question)

**Question**
- Text (Text)
- Type (Enumeration: multiple_choice, true_false, short_answer)
- Correct Answer (Text)
- Points (Number)
- Quiz (Relation to Quiz)

## Project Structure

```
quiz-backend/
├── migration/            # Database migrations
├── src/
│   ├── components/       # Business components
│   │   └── config/       # Configuration services
│   ├── db/               # Database configuration
│   ├── entity/           # Sea-ORM entity models
│   ├── http_response/    # Response builders and utilities
│   ├── security/         # JWT and authentication
│   ├── utils/            # Utility functions
│   └── main.rs           # Application entry point
├── .env                  # Environment configuration
├── Cargo.toml            # Dependencies
├── Dockerfile            # Docker configuration
├── fly.toml              # Fly.io deployment config
└── README.md             # This file
```

## Development Guidelines

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Building Documentation

```bash
cargo doc --open
```

## Authentication & Authorization

The API uses JWT tokens for authentication. Include the token in request headers:

```bash
Authorization: Bearer <your_jwt_token>
```

### Default Permissions

- `quiz.read` - Read quiz data
- `quiz.create` - Create quizzes
- `quiz.update` - Update quizzes
- `question.read` - Read questions
- `question.create` - Create questions
- `session.read` - View quiz sessions
- `session.create` - Start quiz sessions

### Role-Based Access

- **ADMIN**: Full access to all operations
- **TEACHER**: Can create and manage quizzes and questions
- **STUDENT**: Can take quizzes and view results
- **GUEST**: Read-only access to public quizzes

## Deployment

### Fly.io Deployment

1. **Install Fly CLI**
   ```bash
   curl -L https://fly.io/install.sh | sh
   ```

2. **Deploy**
   ```bash
   fly deploy
   ```

## Contributing

1. Create a feature branch: `git checkout -b feature/your-feature`
2. Commit changes: `git commit -am 'Add feature'`
3. Push to branch: `git push origin feature/your-feature`
4. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues, feature requests, or questions, please open an issue on the project repository.

---

**Last Updated**: November 2025
**Status**: Active Development
