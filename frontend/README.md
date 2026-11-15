# AI Football Coach — frontend

TanStack Start web app for coaches. Broader install/run notes live in the [repo README](../README.md). Product docs: [`docs/docs/frontend.mdx`](../docs/docs/frontend.mdx).

## Requirements

- Node.js 26 (see [`.nvmrc`](.nvmrc))

## Install and run

```bash
cd frontend
npm install
npm run dev
```

Dev server: [http://localhost:3000](http://localhost:3000).

## Local backend for `/health`

The `/health` page calls the backend `GET /health` endpoint and shows status (or an unavailable message if the request fails). While the request is in flight it shows a loading state.

1. Start the backend (default [http://127.0.0.1:8000](http://127.0.0.1:8000)):

```bash
cd backend
cargo run
```

2. Start the frontend (`npm run dev` as above).

3. Open [http://localhost:3000/health](http://localhost:3000/health).

By default the page uses backend base URL `http://127.0.0.1:8000` (so it calls `http://127.0.0.1:8000/health`). Override with `VITE_BACKEND_BASE_URL`:

```bash
cd frontend
cp .env.example .env   # optional; gitignored
npm run dev
```

### CORS

The backend allows browser calls from Vite on port 3000 by default (`http://localhost:3000` and `http://127.0.0.1:3000`). To allow other local origins, set `CORS_ALLOWED_ORIGINS` in `backend/.env` (comma-separated). See [`backend/.env.example`](../backend/.env.example).

## Local backend for `/accounts/create`

The `/accounts/create` page submits email and password to the backend `POST /accounts` endpoint and shows success (with the created account email) or a clear error if the request fails (`400` / `409` / unreachable).

1. Start the backend (default [http://127.0.0.1:8000](http://127.0.0.1:8000)) as in the `/health` section above.

2. Start the frontend (`npm run dev`).

3. Open [http://localhost:3000/accounts/create](http://localhost:3000/accounts/create).

By default the page uses backend base URL `http://127.0.0.1:8000` (so it calls `http://127.0.0.1:8000/accounts`). Override with `VITE_BACKEND_BASE_URL` the same way as `/health`. Local CORS notes above apply to browser `POST /accounts` calls as well.

## Tests

```bash
cd frontend
npm test -- --run
```

## Quality gates

```bash
cd frontend
npm run format:check
npm run lint
npm run typecheck
```
