## Harmonify Backend

Harmonify’s backend is a NestJS 11 API that powers authentication, budgeting, tasks, insights, and upcoming sync services. It uses Prisma as the ORM and targets PostgreSQL by default. This README covers local development for the backend component.

## Requirements

- Node.js 22+
- npm 10+
- Docker (optional but recommended for the database)
- PostgreSQL 14+ (local instance or Docker container)

## Environment

The backend reads environment variables from the repository root `.env` file. At minimum define:

```
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/harmonify"
JWT_SECRET="replace-with-strong-secret"
JWT_EXPIRATION="1d"
``` 

Adjust the database URL for your setup. When running through Docker Compose, these values are populated automatically by the root `.env` template.

## Installation

```powershell
cd backend
npm install
```

Generate the Prisma client after installing dependencies:

```powershell
npx prisma generate
```

## Database Migrations

Migrations live in `backend/prisma/migrations`. Apply them using:

```powershell
npx prisma migrate dev --name <migration-name>
```

When working with Docker Compose, run the command inside the backend container:

```powershell
docker-compose exec backend npx prisma migrate dev --name init
```

## Running the Backend

```powershell
# development (auto-reload)
npm run start:dev

# production build
npm run build
npm run start:prod
```

The API serves the health endpoint at `http://localhost:8080/api/ping` by default.

## Testing & Quality

```powershell
# unit tests
npm run test

# e2e tests
npm run test:e2e

# coverage summary
npm run test:cov

# lint (runs ESLint with project config)
npm run lint
```

Add or update tests alongside new modules to keep coverage healthy. For features exposed via the hosted Harmonify Cloud, ensure the OSS backend remains fully functional and documented.

## Project Layout

```
src/
  auth/        Authentication & JWT strategy
  prisma/      PrismaService wrapper and DB utilities
  app.*        Root application wiring
prisma/
  schema.prisma  Database schema and generator config
  migrations/    Generated migration history
```

Domain features will live under `src/modules/<feature>` as they’re introduced (budgeting, tasks, etc.).

## License

Harmonify's backend is distributed under the [GNU Affero General Public License v3.0](../LICENSE). Portions of the NestJS framework remain available under their original [MIT license](https://github.com/nestjs/nest/blob/master/LICENSE). If you deploy a modified backend over a network, Section 13 of the AGPL requires you to make those modifications available to your users.
