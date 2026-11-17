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

### CORS and credentialed cookies

The backend allows browser calls from Vite on port 3000 by default (`http://localhost:3000` and `http://127.0.0.1:3000`) with `Access-Control-Allow-Credentials: true`. To allow other local origins, set `CORS_ALLOWED_ORIGINS` in `backend/.env` (comma-separated). See [`backend/.env.example`](../backend/.env.example).

Vite on `http://localhost:3000` talking to the API on `http://127.0.0.1:8000` is **cross-site**. The session cookie is therefore `HttpOnly; SameSite=None; Secure`, and browser calls use `credentials: 'include'` so the cookie is stored and sent. Chrome treats `http://127.0.0.1` as a secure context for this local topology.

## Local backend for `/accounts/create`

The `/accounts/create` page submits email and password to the backend `POST /accounts` endpoint and shows success (with the created account email) or a clear error if the request fails (`400` / `409` / unreachable).

1. Start the backend (default [http://127.0.0.1:8000](http://127.0.0.1:8000)) as in the `/health` section above.

2. Start the frontend (`npm run dev`).

3. Open [http://localhost:3000/accounts/create](http://localhost:3000/accounts/create).

By default the page uses backend base URL `http://127.0.0.1:8000` (so it calls `http://127.0.0.1:8000/accounts`). Override with `VITE_BACKEND_BASE_URL` the same way as `/health`. Local CORS notes above apply to browser `POST /accounts` calls as well.

## Local cookie session flow: `/login` → `/me` → logout

Browser auth uses **httpOnly `session` cookies** (not JS-readable storage). Frontend calls use `credentials: 'include'`.

Vite (`localhost:3000`) and the API (`127.0.0.1:8000`) are different hosts, so login establishes two cookies with the same opaque token: the API cookie (`SameSite=None; Secure`) for credentialed `/me` / `/logout`, and a frontend-origin cookie (`SameSite=Lax`, via `establishSessionCookie`) so SSR `getCookie('session')` can keep logged-in users on `/me` across full page loads.

1. Start the backend (default [http://127.0.0.1:8000](http://127.0.0.1:8000)) as in the `/health` section above.

2. Start the frontend (`npm run dev`).

3. Create an account at [http://localhost:3000/accounts/create](http://localhost:3000/accounts/create) if you do not already have one.

4. Open [http://localhost:3000/login](http://localhost:3000/login) and sign in. On success both cookies are set and the page shows a logged-in state with the coach email. On `401` / other failure / unreachable backend it shows a clear error and leaves the user logged out.

5. Open [http://localhost:3000/me](http://localhost:3000/me). The SSR guard uses the frontend-origin cookie; the page calls `GET /me` with credentials (API cookie). On success it shows `id` and `email`. On failure it shows a clear error. Logged-out visits redirect to `/login`.

6. Use the **Logout** button on `/me` (or backend `POST /logout`). That clears both cookies and deletes the matching `sessions` row. After logout, `GET /me` with the old API cookie returns `401`.

By default pages use backend base URL `http://127.0.0.1:8000`. Override with `VITE_BACKEND_BASE_URL` the same way as `/health`. CORS / `SameSite=None` notes above apply to the credentialed API calls.

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
