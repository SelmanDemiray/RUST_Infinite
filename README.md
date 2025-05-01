# RTS_RUST

A cross-platform, scalable real-time strategy (RTS) game engine and demo, written in Rust.  
This project separates core logic, server, and WASM client for maximum flexibility and performance.

## Project Structure

- **rts_core**: Shared protocol types, core logic/data structures.
- **rts_server**: Server backend (Axum, SQLx) with REST, WebSocket, and game state management.
- **rts_client_wasm**: WebAssembly client for modern browsers, using Web APIs.
- *(planned)*: Desktop/mobile clients, admin UI, etc.

---

## Deployment Types

RTS_RUST supports several deployment environments:

### 1. Local Development (Recommended for testing)

- **Run the server and client locally.**
- Requires Rust, Node.js (for WASM), PostgreSQL.

#### Setup Steps:
1. **Set up PostgreSQL**  
   Edit `rts_server/.env` with your DB credentials.  
   Start PostgreSQL locally (or use Docker).

2. **Run migrations:**
   ```sh
   cd rts_server
   sqlx migrate run
   ```

3. **Start the server:**
   ```sh
   cargo run -p rts_server
   ```

4. **Build the WASM client:**
   ```sh
   cd ../rts_client_wasm
   wasm-pack build --target web
   ```

5. **Serve the client (static files):**
   ```sh
   basic-http-server .   # or python -m http.server
   ```

6. **Open the game in your browser:**  
   Go to `http://localhost:4000` (or your chosen port).

#### Shutting Down
- **Stop the server:**  
  Press `Ctrl+C` in the terminal running `cargo run`.
- **Stop the static file server:**  
  Press `Ctrl+C` in the terminal running `basic-http-server` or `python -m http.server`.
- **Stop PostgreSQL:**  
  Use your system's service manager or stop the Docker container.

#### Resetting the Project
- **Reset the database (dangerous, erases all data):**
   ```sh
   cd rts_server
   sqlx migrate revert --all
   sqlx migrate run
   ```
- **Remove build artifacts:**
   ```sh
   cargo clean
   cd ../rts_client_wasm
   wasm-pack clean
   ```

---

### 2. Production (Self-Hosted)

- **Deploy server on a cloud VM or container.**
- **Serve WASM client via NGINX or CDN.**
- Use SSL/TLS for WebSocket and API.
- Configure environment variables for DB, address, CORS, etc.

#### Steps:
- Build server and WASM client in release mode.
- Configure and run PostgreSQL.
- Serve static files (`rts_client_wasm/pkg`, `index.html`) with NGINX or similar.
- (Optional) Use Docker for the server and/or database.

#### Shutting Down
- Stop the server process (systemd, Docker, etc.).
- Stop the static file server.
- Stop the database service.

#### Resetting
- Drop and recreate the database, or use migration tools as above.

---

### 3. Docker Compose (Recommended for Integration Tests)

- **Run everything in containers.**
- Add a `docker-compose.yml` with services for `rts_server`, `Postgres`, and a static file server for WASM client.
- Example (not included yet):

```yaml
version: '3'
services:
  db:
    image: postgres:15
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: rts_game
    ports: ["5432:5432"]
  server:
    build: ./rts_server
    environment:
      DATABASE_URL: postgres://user:password@db/rts_game
      SERVER_ADDR: 0.0.0.0:8080
    depends_on: [db]
    ports: ["8080:8080"]
  client:
    image: halverneus/static-file-server
    volumes:
      - ./rts_client_wasm:/web
    ports: ["4000:80"]
```

#### Shutting Down
- Run:
  ```sh
  docker-compose down
  ```

#### Resetting
- Remove all containers, volumes, and networks:
  ```sh
  docker-compose down -v
  ```
- This will erase all database data.

---

### 4. Cloud (Heroku, Fly.io, AWS, etc.)

- **Deploy server and DB using provider's native services.**
- **Serve Web client via static site hosting (S3, Vercel, Netlify, etc.).**
- Use environment variables for secrets/config.
- Ensure CORS is set appropriately on the server.

#### Shutting Down
- Use your provider's dashboard or CLI to stop services.

#### Resetting
- Drop/recreate the database using provider tools.

---

### 5. Desktop Client (Planned)

- **Build a native desktop client using egui, winit, or a game engine.**
- Connects to the same WebSocket API as the browser client.

---

## Notes

- **WebSocket endpoint:** `/ws` (e.g. `ws://127.0.0.1:8080/ws`)
- **REST API:** `/api/register`, `/api/login`
- **Database:** PostgreSQL (see `rts_server/.env` and migrations)
- **Authentication:** Token-based (JWT planned, currently dummy tokens)

---

## Requirements

- Rust (1.70+ recommended)
- wasm-pack (`cargo install wasm-pack`)
- PostgreSQL
- Node.js (optional, for some dev tools)

---

## License

MIT

---
