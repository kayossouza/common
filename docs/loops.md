
# Task 1.4: Define Syntax for Loops

**Objective**: Define the syntax for creating loops in the Common language, ensuring simplicity and flexibility for both beginners and advanced use cases.

## 1. Basic Loop Syntax
- For basic loops, we will use **`do this`** followed by the number of iterations.

**Syntax Example**:
```english
do this 5 times
  display "Hello!"
```

## 2. For-Each Loop
- To iterate over items in a collection, we’ll use **`for every`** followed by the item and collection name.

**For-Each Loop Example**:
```english
for every item in shoppingCart
  display item.name
```

## 3. While Loop
- For loops that run based on a condition, we’ll use the **`while`** keyword in a conversational way.

**While Loop Example**:
```english
while counter is less than 10
  display counter
  change counter to counter plus 1
```

## 4. Loop Control
- The loop control will follow conversational cues such as **`stop`** to break out of a loop early or **`continue`** to skip the current iteration.

**Loop Control Example**:
```english
for every item in shoppingCart
  when item is out_of_stock
    stop
  display item.name
```

## 5. Nested Loops
- Nested loops can be created by adding a second loop inside the block of the first.

**Nested Loop Example**:
```english
for every order in orders
  for every item in order.items
    display item.name
```

## 6. Test Cases

### 6.1 Simple Loop
- Input:
```english
do this 3 times
  display "Running loop"
```
- Expected Output: 
   - The message "Running loop" is displayed 3 times.

### 6.2 For-Each Loop
- Input:
```english
for every item in shoppingCart
  display item.name
```
- Expected Output:
   - The name of each item in the `shoppingCart` is displayed.

### 6.3 While Loop
- Input:
```english
while counter is less than 10
  display counter
  change counter to counter plus 1
```
- Expected Output:
   - The loop runs as long as `counter` is less than 10, displaying the counter's value and incrementing it on each iteration.

### 6.4 Loop Control
- Input:
```english
for every item in shoppingCart
  when item is out_of_stock
    stop
  display item.name
```
- Expected Output:
   - The loop stops as soon as an `out_of_stock` item is encountered.

### 6.5 Nested Loops
- Input:
```english
for every order in orders
  for every item in order.items
    display item.name
```
- Expected Output:
   - The loop iterates over each `order` and, within each order, iterates over `order.items`, displaying the item names.

## 7. Next Steps
- **Task 1.5**: Define Syntax for Error Handling (to create simple, conversational error handling in Common).
