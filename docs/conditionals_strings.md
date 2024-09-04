
# Task 1.3: Define Syntax for Conditionals and Strings

**Objective**: Define the syntax for handling conditional logic and string manipulation in the Common language, ensuring the syntax is conversational and flexible.

## 1. Conditional Statement Syntax
- Conditional statements will be introduced using **`when`** to represent the conditional block.
- The **`else`** keyword will handle the fallback condition.

**Syntax Example**:
```english
when age is greater than 18
  display "You are an adult"
else
  display "You are a minor"
```

## 2. Comparison Operators
- The language will support both conversational and symbolic comparison operators:
  - **"is greater than"** or **`>`**
  - **"is smaller than"** or **`<`**
  - **"is equal to"** or **`==`**
  - **"is not equal to"** or **`!=`**
  - **"is equal or smaller than"** or **`<=`**
  - **"is equal or bigger than"** or **`>=`**

**Examples**:
```english
when score is greater than 50
  display "You passed"
  
when score > 50
  display "You passed"
```

## 3. Logical Operators
- The language will support both conversational and symbolic logical operators:
  - **"and"** or **`&&`**
  - **"or"** or **`||`**

**Logical Operators Example**:
```english
when age is greater than 18 and is_logged_in is true
  display "Access granted"
else
  display "Access denied"
```

## 4. Nested Conditionals
- Nested conditionals are handled by simply adding another **`when`** block inside the current block.

**Nested Conditionals Example**:
```english
when user_role is "admin"
  display "Welcome, Admin"
  when admin_level is greater than 5
    display "You have super admin access"
```

## 5. String Interpolation with `{}`
- Strings are enclosed in single quotes (`'`) for basic strings, and backticks (\`\`) are used for strings containing variables. 
- Variables inside strings are flagged using curly braces (`{}`).

**Simple String Example**:
```english
message is 'Hello, world!'
```

**String with Variable**:
```english
message is `Hello, {name}!`
```

## 6. Test Cases

### 6.1 Simple Conditional
- Input:
```english
when age is greater than 18
  display "You are an adult"
else
  display "You are a minor"
```
- Expected Output:
   - If `age > 18`, the message "You are an adult" will display. Otherwise, "You are a minor" will display.

### 6.2 Symbolic Comparison
- Input:
```english
when score > 50
  display "You passed"
else
  display "You failed"
```
- Expected Output:
   - If `score > 50`, "You passed" will display. Otherwise, "You failed" will display.

### 6.3 Logical Conditions
- Input:
```english
when age > 18 and is_logged_in is true
  display "Access granted"
else
  display "Access denied"
```
- Expected Output:
   - If `age > 18` and `is_logged_in` is `true`, "Access granted" will display. Otherwise, "Access denied" will display.

### 6.4 Nested Conditional
- Input:
```english
when user_role is "admin"
  display "Welcome, Admin"
  when admin_level > 5
    display "You have super admin access"
```
- Expected Output:
   - If `user_role` is `"admin"` and `admin_level > 5`, "You have super admin access" will display. Otherwise, only "Welcome, Admin" will display.

### 6.5 String Interpolation
- Input:
```english
message is `Hello, {name}!`
```
- Expected Output:
   - The variable `name` will be replaced with its value inside the string, displaying "Hello, John!" if `name` is `"John"`.

## 7. Next Steps
- **Task 1.4**: Define Syntax for Loops (to create simple, conversational loops in Common).
