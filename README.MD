# Harmonify: Your Open-Source Holistic Life Management Platform

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Node.js Version](https://img.shields.io/badge/Node.js-22%2B-blue.svg)](https://nodejs.org/)
[![NestJS Version](https://img.shields.io/badge/NestJS-10%2B-red.svg)](https://nestjs.com/)
[![Next.js Version](https://img.shields.io/badge/Next.js-14%2B-black.svg)](https://nextjs.org/)
## Vision

Harmonify is an open-source, holistic life management platform designed to empower individuals to manage every aspect of their lives in one unified app. Inspired by the need to have a comprehensive life tracker that you host and use without sharing your data, Harmonify will start with core features for manual data entry and grow into a holistic life management platform.

> **📢 Important Note:** In the future, this project will transition to its own dedicated organization (e.g., HarmonifyOrg) for better community governance and scalability.

The goal is to create a privacy-first, customizable tool where users can:
- Track finances, tasks, and goals manually (no third-party integrations initially).
- Get personalized insights like daily briefings and analytics.
- Export data easily (e.g., to Excel/CSV).
- Build on a modular foundation for future expansions like health tracking or AI coaching.

Think of it as your digital life hub: Budget your money, plan your day, track habits, and visualize progress—all in one place. We're starting web-first, with mobile apps planned later.

## Key Features

### Core Features (MVP Focus)
These are the foundation—built for manual user input and simple exports:
1. **Budgeting**: Create and manage budgets with categories.
2. **Task Management**: Add, prioritize, and track tasks with reminders.
3. **Savings Tracking**: Monitor savings goals and progress.
4. **Account/Deposit Tracking**: Manually log bank accounts, deposits, and balances.
5. **Income & Expense Tracking**: Log transactions, categorize, and view summaries (export to Excel/CSV).
6. **Investment Tracking**: Manual entry for stocks/portfolios with basic performance views.
7. **Credit & Debt Management**: Track debts, payments, and scores manually.
8. **Subscription Management**: List and remind about recurring payments.
9. **Goal Setting & Tracking**: Set SMART goals and monitor progress.
10. **Analytics & Dashboard**: Basic reports, charts, and life balance overviews.
11. **Personalized Daily Briefing**: Aggregated summary of tasks, events, and reminders.
12. **Predictive Reminders**: Rule-based alerts (e.g., low budget warnings).
13. **Virtual Coach**: Simple tips based on your data (e.g., "Save more this month").

### Future Expansions (Built on Core)
These will be added as modules:
- Journaling, habit tracking, learning progress.
- Vision boards, mind maps for ideas.
- Health features: Exercise, nutrition, sleep, water intake, meditation, symptom tracking.
- Lifestyle tools: Event planning, shared calendars, contacts manager, chores scheduler, shopping lists, inventory (e.g., pantry/wardrobe), travel planner, pet care, smart home integrations.
- Career/networking trackers, creative spaces (sketches, notes).
- Advanced AI: ML-based predictions, full virtual coach.

All features emphasize user privacy—data stays local or self-hosted. No external APIs for finance initially; everything is manual entry like classic money manager apps.

## Tech Stack
- **Backend**: NestJS (structured APIs) with Prisma ORM.
- **Database**: PostgreSQL.
- **Frontend**: Next.js.
- **Deployment**: Docker Compose.
- **Why This Stack?** Scalable, open-source, and community-friendly. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Getting Started

### Prerequisites
- Node.js 22+, Docker Desktop, Git.

### Installation
1. Clone: `git clone https://github.com/Collins-Omariba/harmonify.git && cd harmonify`
2. Copy `.env.example` to `.env`.
3. Run: `docker-compose up --build`
   - Frontend: http://localhost:3000
   - Backend: http://localhost:8080/api/ping

Migrate DB: `docker-compose exec backend npx prisma migrate dev --name init`

## Contributing
We welcome contributions! Whether it's bug fixes, new modules, or UI improvements:
- Check [issues](https://github.com/Collins-Omariba/harmonify/issues) for tasks.
- Fork, create a branch, and submit a PR.
- Follow our [code of conduct](CODE_OF_CONDUCT.md).
- Focus on modularity—each feature as a Django app.

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License
GPLv3 License.

## Roadmap
- MVP Release: Core finance/productivity features.
- Phase 2: Health/lifestyle modules.
- Phase 3: AI enhancements and mobile apps.


Built with ❤️ by the contributors.
