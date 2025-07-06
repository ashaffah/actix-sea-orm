# actix-sea-orm

A Rust web application example demonstrating the integration of **Actix Web** with **SeaORM** for database operations and **Tera** for templating. This project also includes QR code generation functionality using the `qirust` library.

## Features

- **CRUD Operations**: Create, read, update, and delete posts using SeaORM with a MySQL database.
- **Pagination and Search**: List posts with pagination and search functionality.
- **Template Rendering**: Uses Tera templates for rendering HTML pages (`index.html.tera`, `new.html.tera`, `edit.html.tera`, `404.html.tera`, `500.html.tera`).
- **QR Code Generation**: Generate QR codes as SVG or PNG images with customizable options (size, color, border, frame style, etc.).
- **RESTful API**: Public routes for QR code generation under `/public/v1`.

## Dependencies

Key dependencies used in the project:

- `actix-web`: Web framework for building the API and handling HTTP requests.
- `sea-orm`: Asynchronous ORM for MySQL database interactions.
- `tera`: Template engine for rendering HTML pages.
- `qirust`: Library for generating QR codes.
- `image`: Image processing for QR code generation.
- `serde`: Serialization and deserialization for handling JSON data.
- `dotenvy`: Loads environment variables from a `.env` file.
- `env_logger`: Logging configuration.

See `Cargo.toml` for the full list of dependencies.

## Setup

1. **Clone the Repository**:

   ```bash
   git clone <repository-url>
   cd actix-sea-orm
   ```

2. **Set Up Environment Variables**:
   Create a `.env` file in the root directory with the following:

   ```
   DATABASE_URL=mysql://<username>:<password>@<host>:<port>/<database>
   ```

   Replace `<username>`, `<password>`, `<host>`, `<port>`, and `<database>` with your MySQL credentials.

3. **Install Dependencies**:

   ```bash
   cargo build
   ```

4. **Run Migrations**:
   Ensure your MySQL database is running, then apply migrations:

   ```bash
   cargo run -p migration -- up
   ```

5. **Seed the Database** (Optional):
   Populate the database with initial data:

   ```bash
   cargo run -p seeder -- up
   ```

6. **Run the Application**:
   Start the Actix Web server:

   ```bash
   cargo run
   ```

   The application will be available at `http://localhost:8080`.

## Usage

### Post Management

- **List Posts**: `GET /`

  - Query parameters: `page`, `per_page`, `search`
  - Example: `http://localhost:8080/?page=1&per_page=10&search=example`
  - Returns a paginated list of posts with optional search filtering.

- **Create Post**: `GET /new` and `POST /`

  - `GET /new`: Renders a form to create a new post.
  - `POST /`: Submits the form to create a post and redirects to `/`.

- **Edit Post**: `GET /{id}` and `POST /{id}`

  - `GET /{id}`: Renders a form to edit the post with the specified `id`.
  - `POST /{id}`: Updates the post and redirects to `/`.

- **Delete Post**: `POST /delete/{id}`
  - Deletes the post with the specified `id` and redirects to `/`.

### QR Code Generation

- Query parameters: `data`, `size`, `color`, `border`, `inner_px`, `frame_style`, `scale`, `ecc`, `bg_color`

- **Generate SVG QR Code**: `POST /public/v1/qr`

  - Payload: `{ "data": "<string>" }`
  - Returns a JSON response with the SVG string.

- **Generate PNG QR Code**: `GET /public/v1/qr`

  - Example: `http://localhost:8080/public/v1/qr?data=https://example.com&size=500x500&color=000000`
  - Returns a PNG image of the QR code.

- **Generate Framed QR Code**: `GET /public/v1/frameqr`
  - Similar query parameters to `/qr`, with additional support for framing (uses `gh.png` from static assets).
  - Returns a PNG image with a framed QR code.
  - Example: `http://localhost:8000/api/public/v1/frameqr?data=https://example.com&color=000000&size=325x325&border=1&inner_px=10&frame_style=&scale=16&ecc=H`

## Templates

The application uses Tera templates located in the templates directory:

- `index.html.tera`: Displays the list of posts.
- `new.html.tera`: Form for creating a new post.
- `edit.html.tera`: Form for editing an existing post.
- `404.html.tera`: Custom 404 error page.
- `500.html.tera`: Custom 500 error page.

Ensure these templates are configured in your `AppsConfig` for proper rendering.

## Error Handling

- **Database Errors**: Returns HTTP 500 with a JSON error message or renders `500.html.tera`.
- **Not Found**: Returns HTTP 404 with `404.html.tera` for invalid post IDs.
- **Template Errors**: Returns HTTP 500 with a generic error message if Tera fails to render.

## Notes

- The QR code generation uses the `qirust` library, which supports customizable QR codes with options like error correction level (`ecc`), frame styles, and colors.
- The application assumes a MySQL database is configured via the `DATABASE_URL` environment variable.
- Static assets (e.g., `gh.png` for framed QR codes) should be placed in `src/static/images/`.

## License

This project is licensed under the MIT License.
