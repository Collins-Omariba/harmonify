# Harmonify: Your Open-Source Holistic Life Management Platform

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Backend](https://img.shields.io/badge/Backend-Rust-orange.svg)](#tech-stack)
[![Frontend](https://img.shields.io/badge/Frontend-Next.js-black.svg)](https://nextjs.org/)
[![Database](https://img.shields.io/badge/Database-PostgreSQL-blue.svg)](https://www.postgresql.org/)
[![Deployment](https://img.shields.io/badge/Deployment-Docker%20Compose-2496ED.svg)](https://www.docker.com/)

## Vision

Harmonify is an open-source, holistic life management platform designed to empower individuals to manage every aspect of their lives in one unified app. Inspired by the need to have a comprehensive life tracker that you can self-host without sharing your data, Harmonify starts with core features for manual data entry and grows into a holistic life management experience.

> **📢 Important Note:** In the future, this project will transition to its own dedicated organization (e.g., HarmonifyOrg) for better community governance and scalability.

The goal is to create a privacy-first, customizable tool where users can:
- Track finances, tasks, and goals manually (no third-party integrations initially).
- Get personalized insights like daily briefings and analytics.
- Export data easily (e.g., to Excel/CSV).
- Build on a modular foundation for future expansions like health tracking or AI coaching.

Think of it as your digital life hub: budget your money, plan your day, track habits, and visualize progress—all in one place. We're starting web-first, with mobile apps planned later.

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
- **Backend**: NestJS (currently) → Rust (migration in progress).
- **Database**: PostgreSQL.
- **Frontend**: Next.js.
- **Deployment**: Docker Compose.
- **Why This Stack?** Scalable, open-source, and community-friendly. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.


## Quick Start


> **⚠️ Development Status**
>
> Harmonify is currently under active development. The backend is being migrated from NestJS to Rust.

### Prerequisites

- Docker
- Docker Compose

### Setup

```bash
git clone https://github.com/yourusername/harmonify.git
cd harmonify

cp .env.example .env

docker compose up --build
```

The application will be available at:

- Frontend: http://localhost:3000
- Backend: http://localhost:8080


## Contributing
We welcome contributions! Whether it's bug fixes, new modules, or UX improvements:
- Check [issues](https://github.com/Collins-Omariba/harmonify/issues) for tasks.
- Fork, create a branch, and submit a PR.
- Follow our [code of conduct](CODE_OF_CONDUCT.md).
- Keep features modular and well-typed for both the NestJS backend and Next.js frontend.

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Harmonify Cloud (Hosted Option)

Want managed sync, secure backups, and automatic updates? Harmonify Cloud (in development) will offer:

- Encrypted sync across devices, powered by the same open-source core.
- Hands-off hosting with daily backups and monitored uptime.
- Early access to premium modules (advanced analytics, AI mentor) while contributions still land in the open repo whenever possible.

Self-hosters always keep full control, and the AGPL ensures improvements made for the hosted service remain available to the community.

## License
Harmonify is released under the [GNU Affero General Public License v3.0](LICENSE). If you operate a modified version over a network, you must make the corresponding source available to users as described in Section 13 of the AGPL.

## Support & Community
- General questions or feedback: [Discussions](https://github.com/Collins-Omariba/harmonify/issues) - (just issues now will be an actual discussion after moving to an org) -
- Security issues: email ----
- Hosted Harmonify Cloud inquiries: ----

## Roadmap
- MVP Release: Core finance/productivity features.
- Phase 2: Health/lifestyle modules.
- Phase 3: AI enhancements and mobile apps.


Built with ❤️ by the contributors.
