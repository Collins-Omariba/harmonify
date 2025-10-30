## Harmonify Frontend

The Harmonify frontend is a Next.js App Router application styled with Tailwind CSS. It provides the dashboard, onboarding, and upcoming premium experiences that will connect to Harmonify Cloud.

## Prerequisites

- Node.js 22+
- npm 10+
- Backend API running locally (via Docker Compose or `npm run start:dev` in `backend/`)

## Installation

```powershell
cd frontend
npm install
```

## Environment Variables

Frontend configuration is read from the root `.env`. Ensure the following variables exist:

```
NEXT_PUBLIC_API_BASE_URL="http://localhost:8080"
```

Add additional public variables as new modules require them. Avoid placing secrets in `NEXT_PUBLIC_*` variables.

## Development Scripts

```powershell
# start the development server with Turbopack
npm run dev

# run lint checks
npm run lint

# production build
npm run build
npm start
```

Visit http://localhost:3000 to view the app.

## Project Structure

```
app/
	layout.tsx        Root layout
	page.tsx          Landing experience
	dashboard/        Authenticated dashboard shell
	login/            Authentication flows
	register/         Registration flow
components/
	LoginForm.tsx     Auth forms using React Hook Form + Zod
	RegisterForm.tsx  Registration form and validation
public/             Static assets
```

Shared UI, hooks, and utility modules should live under `components/` or a soon-to-be-created `lib/` directory. Keep modules self-contained to support white-labelling and premium offerings.

## Testing

Frontend tests are currently limited; contributions adding Jest + Testing Library coverage are welcome. When adding new components, include tests and run them locally before opening a pull request.

## Deployment Notes

- Self-hosted instances can deploy the frontend via Docker Compose (`frontend` service) or any static hosting provider that supports Next.js serverless functions.
- Harmonify Cloud uses the same codebase with additional environment values for managed sync features; keep conditional logic feature-flagged and open-source friendly.

## License

This frontend is licensed under the [GNU Affero General Public License v3.0](../LICENSE). If you host a modified build for others to use, you must provide access to the corresponding source code.
