# Reject finance bill

## Project Overview

This website is created to support and coordinate efforts against the proposed finance bill in Kenya. On the 20th, a demonstration will be held, and this website will be used to track individuals who are arrested during the protest, the police stations they are taken to, and the towns where these events occur.

### Purpose
The primary purpose of this website is to provide a platform for tracking and reporting arrests during the demonstration against the finance bill. It aims to ensure that arrested individuals are accounted for and can receive timely assistance and support.

### Features
**Dashboard:** A central hub displaying key metrics, recent activities, and notifications related to the demonstration and arrests.

**Arrest Help Form:** A form for submitting details of individuals who are arrested, including their username, first name, last name, phone number, location, and police station.

**Tracking System:** A system to monitor and display information about arrested individuals and their current status.

**Resources:** Legal aid and support resources for those affected by the arrests.

**Contact Information:** Easy access to contact details for emergency assistance and support.

## Project Setup

### Prerequisites
Before you begin, ensure you have the following installed on your machine:

- Rust: You can install Rust using rustup.
- PostgreSQL: Ensure you have PostgreSQL installed and running.

### Folder Structure
```
reject-finance-bill/
├── assets/
├── Cargo.lock
├── Cargo.toml
├── database_setup/
├── README.md
├── src/
├── target/
└── templates/
```
- **assets/:** Directory for static assets like CSS, JavaScript, or images.
- **Cargo.lock:** Auto-generated file by Cargo for dependency locking.
- **Cargo.toml:** Manifest file for Rust packages and dependencies.
- **database_setup/:** Directory for database setup scripts or configuration files.
- **README.md:** This file, providing an overview and contribution guidelines.
- **src/:** Main directory for Rust source code files.
- **target/:** Auto-generated directory where compiled binaries and libraries are stored.
- **templates/:** Directory for HTML template files used in the project.

### Running the project locally

Create a `.env` file with the following:
```
PORT="3000"
BASE_ADDRESS="0.0.0.0"
DATABASE_URL=postgresql://postgres:password@localhost:5432/civicdatabase

```
On your postgress run the init.sql inside database_setup directory with the database name civicdatabase.

Then run
```
cargo watch -x run -w templates -w src
```


### Submitting Changes
To contribute to the project, follow these steps:

**1. Fork the Repository**

Fork the repository on GitHub.

**Create a Branch**

Create a new branch for your feature or bugfix:

```sh
git checkout -b feature-name
Make Changes
```

Implement your feature or bugfix in your branch.

**Commit Changes**

Commit your changes with a meaningful commit message:

```sh
git commit -m "Description of the feature or fix"
Push Changes
```

**Push your changes to your forked repository:**

```sh
git push origin feature-name
Create a Pull Request
```

**Open a pull request from your forked repository to the main repository.**
