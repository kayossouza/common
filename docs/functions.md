
# Task 1.2: Define Syntax for Functions

**Objective**: Define a clear, simple, and conversational syntax for creating and invoking functions in the Common language.

## 1. Function Declaration Syntax
- Functions are declared using the conversational keyword **`create a function called`**.
- Parameters are specified after **`with`**, and functions can handle multiple parameters.

**Syntax Example**:
```english
create a function called greet with name
  display "Hello, " and name
```

## 2. Function Invocation
- Functions are called simply by referencing their name and passing the required arguments.

**Invocation Example**:
```english
greet "John"
```

## 3. Handling Multiple Parameters
- Multiple parameters are handled conversationally, listing them after the **`with`** keyword.

**Multiple Parameters Example**:
```english
create a function called add_numbers with a and b
  return a plus b
```

- Invoking the function:
```english
add_numbers 5 and 10
```

## 4. Return Values
- Functions return values using the keyword **`return`**.

**Return Value Example**:
```english
create a function called double with number
  return number times 2
```

## 5. Arithmetic Operations
- The language supports both conversational terms **`plus, minus, times`** and symbols **`+ - *`** for arithmetic operations.

**Using Conversational Arithmetic**:
```english
create a function called double with number
  return number times 2
```

**Using Symbols**:
```english
create a function called double with number
  return number * 2
```

## 6. Void Functions
- If a function does not return anything, it simply executes its block without a return statement.

**Void Function Example**:
```english
create a function called log_message with message
  display message
```

## 7. Test Cases

### 7.1 Simple Function Declaration
- Input:
```english
create a function called greet with name
  display "Hello, " and name
```
- Expected Output: 
   - A function called `greet` is created, taking a single parameter `name`.

### 7.2 Function Invocation
- Input:
```english
greet "John"
```
- Expected Output:
   - The function `greet` is called with the argument `"John"`, and the output is `Hello, John`.

### 7.3 Function with Multiple Parameters
- Input:
```english
create a function called add_numbers with a and b
  return a plus b
```
- Expected Output: 
   - A function called `add_numbers` is created with two parameters, `a` and `b`.

### 7.4 Function Return Value
- Input:
```english
double 4
```
- Expected Output:
   - The function `double` returns `8` when called with the argument `4`.

## 8. Next Steps
- Further tasks for testing, including function recursion and advanced use cases.
