# Rust Web App with Actix-web, Tera, and HTMX

This is a modular Rust web application built using `Actix-web` for the backend, `Tera` for templating, and `HTMX` for client-side interactivity. It features structured routing, authentication, and logging with `tracing`. The app serves static files, handles dynamic page rendering, and integrates easily with databases.

## Features

- **Backend:** Built with `Actix-web` for high-performance web server.
- **Templating:** Uses `Tera` for flexible HTML rendering.
- **Dynamic Interactivity:** Uses `HTMX` to add dynamic behavior without JavaScript frameworks.
- **Logging:** Integrated with `tracing` and `tracing-actix-web` for structured logging.
- **Modular Structure:** Separated concerns for web routes, authentication, and API management.
- **Error Handling:** Custom 404 pages with detailed error reporting.
- **Static Files:** Serving static files (CSS, JS, images) with `actix-files`.

## Prerequisites

To run this project, you need the following:

- [Rust](https://www.rust-lang.org/learn/get-started) installed (preferably with `rustup`).
- [Cargo](https://doc.rust-lang.org/cargo/) to manage Rust dependencies.

## Getting Started

1. **Clone the repository:**

   ```bash
   git clone https://github.com/IAmKushagraSharma/tda-web.git
   cd tda-web
   ```

2. **Install dependencies:**

   The dependencies will be installed automatically when you run the application, but you can update them with:

   ```bash
   cargo build
   ```

3. **Set up environment variables:**

   Create a `.env` file in the root of the project and define any necessary environment variables, such as:

   ```bash
   DATABASE_URL=your_database_url
   SECRET_KEY=your_secret_key
   ```

4. **Run the app:**

   Start the server with the following command:

   ```bash
   cargo run
   ```

   By default, the server will be running on `http://127.0.0.1:3000`.

## Project Structure

- **`src/main.rs`**: The entry point for the application, where Actix-web is configured.
- **`src/routes/`**: Contains route definitions for different parts of the application, such as API, web pages, and authentication.
- **`src/services/`**: Business logic and services for interacting with the database or handling specific tasks.
- **`src/models/`**: Contains data models, such as `User`, `Post`, etc.
- **`src/utils/`**: Utility functions for validation, formatting, etc.
- **`templates/`**: HTML templates for rendering pages (Tera templates).
- **`static/`**: Directory for static files like CSS, JS, and images.
- **`logs/`**: Log files generated during app execution.
- **`config/`**: Configuration files for different environments (development, production).

## HTMX Integration

This project uses `HTMX` to enhance user interaction with server-side rendering. You can make dynamic requests (GET, POST) to the server without needing to reload the page. Here's an example of how HTMX is used:

```html
<button hx-get="/some-endpoint" hx-target="#response">Fetch Data</button>
<div id="response"></div>
```

When the button is clicked, an HTTP request is made to `/some-endpoint`, and the response will be inserted into the `#response` div.

## Logging

The application uses `tracing` and `tracing-actix-web` to handle structured logging. It logs requests, errors, and other important events. Logs are output to the console and can be configured to be written to a file.

Log levels:

- `INFO`: General information about the app's state.
- `ERROR`: Logs errors encountered during execution.
- `TRACE`: Detailed logs for debugging.

### Example log output

```
[INFO] 2024-11-29T10:00:00Z: GET /about
[ERROR] 2024-11-29T10:00:01Z: Database connection failed
```

## Custom 404 Pages

If a user navigates to a non-existent route, a custom 404 page will be rendered, ensuring a smooth user experience with relevant error information.

## Contributing

We welcome contributions! If you'd like to contribute to this project:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make the changes and commit them.
4. Push your branch and open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```

### Key Sections:
1. **Features** - Highlights the primary features of the application.
2. **Prerequisites** - Lists what is needed to run the app.
3. **Getting Started** - A step-by-step guide on setting up the app locally.
4. **Project Structure** - Explains the folder structure and where different components are located.
5. **HTMX Integration** - Describes how HTMX is used for dynamic interactions.
6. **Logging** - Explains the logging system using `tracing`.
7. **Custom 404 Pages** - Mentions the custom error pages for non-existent routes.
8. **Contributing** - Provides instructions for contributing to the project.
9. **License** - Provides licensing information.

This README covers all important aspects of your project and should be comprehensive for anyone who wishes to understand, use, or contribute to your app.
```
