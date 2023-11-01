[![lint](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/lint.yml)
[![build](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/build.yml/badge.svg)](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/build.yml)
[![test](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/test.yml/badge.svg)](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/test.yml)
[![format](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/format.yml/badge.svg)](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/format.yml)


IDS706_DataEngineering_BarbaraFlores_Project2


## 📂 Rust CLI Binary with SQLite

The objective of this project is to develop a Rust command-line application (CLI) that interacts with an SQLite database. This involves creating and manipulating the database, using GitHub Copilot, generating an optimized binary as a GitHub Actions artifact, properly configuring GitHub Actions to test, build, and lint the Rust code, and producing a demonstration video explaining and showcasing the project's functionality.


### 📊 Database - SQLite

In particular, the [world-small.csv](https://raw.githubusercontent.com/sejdemyr/sejdemyr.github.io/master/r-tutorials/basics/data/world-small.csv) database was used, which was employed in the `"Practical Data Science"` class taught by Nick Eubank. This database contains information about some countries, their regions, and their values for `Polity IV` and `gdppcap08`.

- The `polityIV` variable in this dataset is an expert score for a country's authoritarianism. Traditionally, values of -10 represent extreme autocracies, while values of 10 denote consolidated liberal democracies. However, in this dataset, they have been rescaled to range from 0 to 20, where 0 represents an extreme autocracy, and 20 represents a consolidated liberal democracy.

- The variable `gdppcap08` represents the GDP per Capita values for countries in the year 2008.

At the core of this project lies a robust SQLite database, instrumental in showcasing the versatility of CRUD (Create, Read, Update, Delete) operations. The project initially began with a [CSV data source](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/blob/main/data/WorldSmall.csv), which was meticulously transformed and efficiently managed using this powerful database.

### 🔧 CRUD Operations

The project is a Rust command-line application designed for seamless interaction with an SQLite database, offering essential CRUD functionality. It empowers users to efficiently manage and manipulate data, showcasing the versatility of this Rust CLI application. The central hub for coordinating these operations is the [`main.rs`](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/blob/main/src/main.rs) file, which orchestrates the CREATE, UPDATE, READ, and DELETE operations. Here's a closer look at what each of these operations entails:


- **CREATE**
  In the [create.rs](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/blob/main/src/create.rs) file, the CREATE operation involves creating a database from a CSV file. It accomplishes this by using the `create::create_database_from_csv` function. This function connects to an SQLite database, creates a table, and then reads data from the provided CSV file, inserting it into the table.

- **UPDATE**
  The UPDATE operation is carried out using the [update.rs](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/blob/main/src/update.rs) file, which uses the `update::update_table` function. It transforms the database table, converting specific columns, such as 'country' and 'region,' to uppercase.

- **READ**
  The READ operation, defined in the [read.rs](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/blob/main/src/read.rs) file, performs queries on the database. In the project, two sample queries are demonstrated. The first query selects a random sample of five records from the database and displays them. The second query groups records by the 'region' column, counting the number of records in each region, and then displays the results.

- **DELETE**
  The DELETE operation, managed by the [delete.rs](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/blob/main/src/delete.rs) file, is responsible for deleting the SQLite database file. It waits for a duration before performing the deletion. If the database file exists, it is removed.


### 🚀 Optimized Rust Binary

This project goes beyond a basic Rust command-line application. As part of the optimization process, an optimized Rust binary is generated as a GitHub Actions artifact. This optimized binary ensures efficient execution and performance, making it ideal for a wide range of applications. Users can download the optimized binary directly from the GitHub Actions artifacts, ensuring they have a high-performing application ready for use.

The following process is used to generate an optimized Rust binary:

- The `cargo build --release` command is used to build the Rust binary in release mode.
- The `Makefile` is edited to include optimization settings and additional build configurations.
- The `Cargo.toml` file is edited to include necessary dependencies and configurations for optimization.
- The optimized Rust binary is generated as a GitHub Actions artifact that can be downloaded.


### 🤖 GitHub Copilot Assistance
GitHub Copilot, an AI-powered coding assistant, played a significant role in the development of this Rust CLI application. It assisted in generating code snippets, suggesting best practices, and improving the overall code quality. Specifically, it provided valuable support in the following areas:

- Code Generation: GitHub Copilot helped generate boilerplate code for creating, updating, reading, and deleting operations, making the development process more efficient.
- Error Handling: It offered suggestions for effective error handling, ensuring that the application could gracefully handle potential issues.
- Documentation: GitHub Copilot assisted in writing clear and concise comments, making the codebase more understandable for developers.
- Testing: It provided guidance on writing unit tests to ensure the reliability of the application.
- Optimization: Copilot suggested optimizations to enhance the performance of the Rust binary generated by the project.

By leveraging GitHub Copilot's capabilities, the development process was streamlined, resulting in a well-structured and efficient Rust CLI application that interacts with an SQLite database.


### 📚User Instructions

1. Clone this repository:
   
   Go to the [repository page](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/tree/main), then click on "Use this template," and then "Create New Repository."

![Step 1: Clone the repository](https://raw.githubusercontent.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/main/images/Step01.png)


2. Create a Codespace
   
3. Install Rust in Codespace using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
1
source "$HOME/.cargo/env"
```

4. Make sure you have Rust installed on your system, using:
   
```bash
rustc --version
cargo --version
```
![Step 2: Clone the repository](https://raw.githubusercontent.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/main/images/Step02.png)

5. Install Dependencies:

   The dependencies for this project are:
- `rusqlite` = "0.25.2"
- `prettytable` = "0.10.0"
- `csv = "1.3"`
These dependencies are already specified in our `Cargo.toml` file, so to install them, you simply need to execute:

   ```bash
   cargo build
   ```

6. Execute the project:
   
   Use the following command to run the application:

   ```bash
   cargo run
   ```
7. Building the Binary
   
   To build the optimized Rust binary for better performance, use the following command:

   ```bash
   bash cargo build --release
   ```
   Building in release mode optimizes the binary for performance, making it more efficient and suitable for production use.
   
Now you're ready to use the project!


### 📷 Execution Examples

Here are some examples of what you can expect when you run this project:
![Example](https://raw.githubusercontent.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/main/images/Example.png)



### 📊 Video Tutorial
The following [YouTube link](https://youtu.be/7IdvxX5F508) shows a clear, concise walkthrough and demonstration of the CLI binary

