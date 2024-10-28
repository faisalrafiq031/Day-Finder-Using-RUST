# Day-Finder-Using-RUST

<h2><b>Date Day Finder in Rust</b></h2>

This Rust program is designed to determine the day of the week for any given date provided by the user. By asking the user to input a date in the format dd/mm/yyyy, it calculates and returns the day of the week (e.g., Monday, Tuesday) for that specific date. The program is built with robust error handling to ensure it validates the date format and input values before performing any calculations, making it a reliable tool for date-related tasks.

<h2>Key Features:</h2>

<li><b>Date Parsing and Validation:</b> Uses the chrono crate's NaiveDate and Datelike modules to parse, validate, and handle dates efficiently.</li>
<li><b>User-Friendly Input Prompt:</b> Provides a prompt for users to input a date in dd/mm/yyyy format and ensures the input is correctly formatted.</li>
<li><b>Accurate Day Calculation:</b> Determines the day of the week for any valid date input, including handling of leap years.</li>
<li><b>Error Handling:</b> The program includes error messages to handle incorrect date formats, such as invalid days (e.g., 31/02/2023) or input formats that do not match dd/mm/yyyy.</li>

<h2>How the Program Works</h2>

**Input Prompt:**

The program prompts the user to enter a date in the format dd/mm/yyyy and uses io::stdout().flush() to ensure the prompt is displayed immediately.

**Reading and Parsing the Input:**

It reads the input using io::stdin().read_line(&mut input).
After reading the input, it splits the string by / into three parts representing day, month, and year.

**Validating Date Format:**

The program checks that exactly three parts were entered.
Each part is then parsed as an integer. If the conversion fails, it provides an error message indicating that the input is invalid.

**Date Conversion Using NaiveDate:**

Using **NaiveDate::from_ymd_opt(year, month, day)**, it attempts to create a valid date. This function returns an Option type:

If Some **(date)** is returned, the date is valid.
If None is returned, the date is invalid **(e.g., if it’s an impossible date like February 31st).**

**Determining the Day of the Week:**

For valid dates, the program calculates the day of the week using **date.weekday()** from the Datelike trait.
The weekday is then printed in a user-friendly message, indicating which day the date falls on.

**Error Handling for Invalid Inputs:**

If the input format is incorrect (not dd/mm/yyyy), or if the date is invalid, the program prints an appropriate error message and exits.

<h3>Example Usage</h3>

**Correct Input:**

<pre><code>Enter a date (dd/mm/yyyy): 25/10/2023
The day on 25/10/2023 was a Wednesday</code></pre>

**Incorrect Format or Invalid Date:**

<pre><code>Enter a date (dd/mm/yyyy): 31/02/2023
Invalid date provided. Please ensure the date is valid.</code></pre>
