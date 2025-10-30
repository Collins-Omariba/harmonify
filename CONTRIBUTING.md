# Contributing to Harmonify

Thanks for your interest in helping build Harmonify—the privacy-first life management platform. By contributing, you agree that all contributions are provided under the [GNU Affero General Public License v3.0](LICENSE). If your changes introduce services that are accessed over a network, remember that the AGPL requires you to publish the corresponding source when you deploy them.

## 📬 Ways to Contribute

- **Report issues** – Prefer reproducible steps and logs/screenshots when relevant.
- **Pitch improvements** – Share ideas for features, documentation, or developer experience.
- **Tackle bugs & features** – Grab a good first issue or propose a plan for larger work.
- **Polish docs** – Clarify setup, add diagrams, or translate guides.
- **Strengthen tests** – Add coverage or improve tooling for backend (NestJS) and frontend (Next.js).

## 🧭 Project Structure at a Glance

```
backend/   NestJS + Prisma API and auth layer
frontend/  Next.js app router UI
docker-compose.yml  Local development orchestration (PostgreSQL + services)
prisma/    Database schema and migrations
```

The open-source core covers self-hosted use. A hosted Harmonify Cloud will offer optional sync and managed infrastructure for a small fee. Contributions that keep the open core excellent are welcome, and improvements that help the hosted service scale should land here too when they benefit self-hosters.

## 🛠️ Getting Set Up

1. **Fork & clone**
   ```powershell
   git clone https://github.com/<your-username>/harmonify.git
   cd harmonify
   git remote add upstream https://github.com/Collins-Omariba/harmonify.git
   ```
2. **Install prerequisites**
   - Node.js 22+
   - Docker Desktop (for PostgreSQL + services)
   - npm (ships with Node) or your preferred compatible package manager
3. **Configure environment variables**
   - Duplicate the provided `.env.example` into `.env` at the repository root and fill in the values (database host, JWT secrets, etc.).
   - Ensure `DATABASE_URL` points at the PostgreSQL service from Docker Compose (e.g. `postgresql://postgres:postgres@localhost:5432/harmonify`).
4. **Install dependencies**
   ```powershell
   cd backend
   npm install
   cd ..\frontend
   npm install
   cd ..
   ```
5. **Start services**
   ```powershell
   docker-compose up --build
   ```
   - Backend: http://localhost:8080/api/ping
   - Frontend: http://localhost:3000
6. **Apply database schema**
   ```powershell
   docker-compose exec backend npx prisma migrate dev --name init
   ```

## 🧑‍💻 Development Workflow

- Create a feature branch such as `feat/auth-audit-logging`.
- Prefer incremental, focused commits with clear messages (e.g. `fix: ensure jwt guard blocks inactive users`).
- Keep pull requests scoped; open a draft PR early for feedback when work is in progress.
- Document behavioral changes in the relevant README or `/docs` content.

## 🧹 Code Quality & Standards

- **TypeScript everywhere** – Both backend and frontend are written in TypeScript. Strive for strong types.
- **Formatting** – `npm run format` in `backend/` (Prettier). Use your editor’s Prettier integration for the frontend—run `npm run lint` before committing.
- **Linting** –
  - Backend: `npm run lint`
  - Frontend: `npm run lint`
- **Testing** –
  - Backend unit tests: `npm run test`
  - Backend e2e tests: `npm run test:e2e`
  - Frontend tests (add as implemented): ensure new components include Jest/React Testing Library coverage when appropriate.
- **Database** – Regenerate Prisma client after schema changes: `npx prisma generate`. Include generated migration files in your PR.

## ✅ Pre-PR Checklist

Before you open a pull request:

- [ ] Rebase on the latest `main` and resolve conflicts locally.
- [ ] Ensure `npm run lint` succeeds for any package you touched.
- [ ] Run automated tests relevant to your change (`npm run test`, etc.).
- [ ] Update documentation, environment examples, and TypeScript types when behavior changes.
- [ ] Verify new endpoints/components include AGPL-friendly notices if they surface hosted-only capabilities.

## 🔄 Pull Request Process

1. Fill out the PR template (or provide summary/validation details if no template yet).
2. Link related issues (e.g. `Closes #42`).
3. Respond promptly to review feedback. It’s fine to push follow-up commits; squash is handled on merge.
4. After approval, a maintainer will merge. Avoid merging your own PRs unless you’re a maintainer following project policy.

## 🤝 Code of Conduct & Support

Please uphold our [Code of Conduct](CODE_OF_CONDUCT.md) in every interaction.

For broader questions, use [GitHub Discussions](https://github.com/Collins-Omariba/harmonify/issues) - (just issues now will be an actual discussion after moving to an org) -.

We’re excited to build Harmonify together—thank you for your contributions!