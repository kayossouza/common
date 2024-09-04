
# Task 1.1: Define Syntax for Variables

**Objective**: Define a clear, simple, and conversational syntax for declaring and assigning variables in the new universal language.

## 1. Syntax Design

### 1.1 Variable Declaration Syntax
- The syntax for declaring and assigning variables will follow a natural language structure using the word **`is`** for assignment.
- Variables will be declared using **snake_case** to ensure consistency and readability.

**Syntax Example**:
```english
age is 25
username is "JohnDoe"
is_logged_in is true
```

### 1.2 Variable Types
- The language will support **dynamic typing**, meaning variable types are inferred from their assigned values.
- Types include: **strings**, **numbers**, **booleans**, **objects**, and **arrays**.

**Examples**:
```english
username is "John"       # string
age is 30                # number
is_logged_in is false    # boolean
user_data is { name: "John", age: 25 }  # object
shopping_cart is [ "item1", "item2" ]   # array
```

## 2. Type Inference and Safety
- The language will infer the type of variables based on the value provided at the time of assignment.
- Optional type hinting can be introduced for advanced use cases, though not mandatory for beginners.

**Example with Optional Type Hinting**:
```english
age is 25 as number
username is "JohnDoe" as string
```

## 3. Variable Reassignment
- Variables can be reassigned using the conversational keyword **`change`** followed by the new value.

**Reassignment Example**:
```english
change age to 30
change is_logged_in to false
```

## 4. Constant Variables
- A constant variable can be declared using **`is constant`**, meaning the value cannot be reassigned.

**Constant Variable Example**:
```english
pi is constant 3.14159
```

## 5. Scope
- Variables declared at the **global level** will be accessible across the entire application.

**Global Variable Example**:
```english
set global user_data to { name: "John", age: 25 }
```

- **Local variables** declared within a function or component are only accessible in that scope.

**Local Variable Example**:
```english
create a function called greet
  set local greeting_message to "Hello, world!"
```

## 6. Test Cases

### 6.1 Simple Variable Assignment
- Input:
```english
age is 25
```
- Expected Output: 
   - Variable `age` is created and assigned the value `25`.

### 6.2 Reassignment
- Input:
```english
change age to 30
```
- Expected Output:
   - Variable `age` is updated with the new value `30`.

### 6.3 Type Inference and Reassignment
- Input:
```english
is_logged_in is true
change is_logged_in to false
```
- Expected Output:
   - `is_logged_in` is first assigned a boolean value (`true`).
   - The value is later updated to `false`.

### 6.4 Constant Variable
- Input:
```english
pi is constant 3.14159
```
- Expected Output:
   - The constant variable `pi` is assigned the value `3.14159`, and reassignment is disallowed.

## 7. Next Steps
- **Task 1.2**: Define Syntax for Functions (to ensure a consistent syntax for functions alongside variable declarations).
- Further tests for type inference and scope.
