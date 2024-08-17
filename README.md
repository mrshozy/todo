# Todo API with Rust, Axum, and SQLite

This project is a RESTful API for managing todo items, built with Rust using the Axum web framework and SQLite as the database. The API supports operations such as creating, reading, updating, and deleting (CRUD) todo items.

## Features

- **Create a Todo**: Add a new todo item with a title, description, due date, priority, and completion status.
- **Get a Single Todo**: Retrieve a specific todo item by its ID.
- **Update a Todo**: Modify an existing todo item.
- **Delete a Todo**: Remove a todo item by its ID.

## Technology Stack

- **Rust**: The programming language used to build the application.
- **Axum**: A web framework for building asynchronous applications in Rust.
- **SQLite**: A lightweight, file-based database.
- **SQLx**: An async SQL toolkit for Rust.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.54.0 or later)
- [SQLite](https://www.sqlite.org/download.html)

### Clone the Repository

```bash
git clone git@github.com:mrshozy/todo.git
```
```bash
cd Todo
```

## Install Dependencies
Ensure that you have the required Rust dependencies installed:

```bash
cargo build
```
## Run the Application
Start the server by running:

``` bash
cargo run
```
The server will start on http://localhost:3000.

## API Endpoints

### Create a Todo
* URL: /todos
* Method: POST
* Request Body:
```json
{
    "title": "Example Todo",
    "description": "This is a sample todo item",
    "due_date": "2023-11-24T18:00:00",
    "priority": 1
}
```
* Response: 201 Created

### Get a Single Todo
* URL: /todos/{id}
* Method: GET
* Response: 200 OK
* Response body:
```json
{
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "Example Todo",
    "description": "This is a sample todo item",
    "due_date": "2024-08-20",
    "completed": false,
    "priority": 1,
    "created_at": "2024-08-17 12:00:00",
    "updated_at": "2024-08-17 12:00:00"
}
```
### Update a Todo
* URL: /todos/{id}
* Method: PUT
* Request Body:
* Response: 200 OK
```json
{
    "title": "Updated Todo",
    "description": "This todo has been updated",
    "due_date": "2024-08-21",
    "completed": true,
    "priority": 2
}
```

### Delete a Todo
* URL: /todos/{id}
* Method: DELETE
* Response: 204 No Content

## Error Handling

```json
{
    "code": number,
    "error": errorCode,
    "message": errorMessage
}
```

`404` Not Found: Returned if the requested resource (e.g., a specific todo item) is not found.

`409` Conflict: Returned if the provided fields already exist on the database.

`500` Internal Server Error: Returned if there is a server-side error, such as a database failure.

## Database

The application uses SQLite as the database, with a todos table that has the following schema:

```sql
CREATE TABLE IF NOT EXISTS todos (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    due_date TEXT,
    completed BOOLEAN NOT NULL DEFAULT 0,
    priority INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```
## License

This project is licensed under the [MIT License](LICENSE). See the `LICENSE` file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.