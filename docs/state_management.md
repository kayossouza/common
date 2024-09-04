# State Management and Conditional Syntax Documentation

## 1. State Management

### Global State

- **Declaration**: 
  To declare a global state, use the `$` symbol followed by the state name. The state is defined with an initial value enclosed in `{}`.
  
  ```plaintext
  $global_state is { count: 0, user: null }
  ```

- **Update**: 
  To update a global state, use the `update` keyword followed by the state name and the new value.
  
  ```plaintext
  update $global_state count to 5
  ```

### Local State

- **Declaration**: 
  To declare a local state, use the `#` symbol followed by the state name. This is scoped to the component or function where it's defined.
  
  ```plaintext
  #local_state is { count: 0, status: 'idle' }
  ```

- **Update**: 
  To update a local state, use the `update` keyword followed by the local state name and the new value.
  
  ```plaintext
  update #local_state count to 10
  ```

### Context-Like Usage

- **Providing Global State**: 
  To provide a global state to a component or function, use the `provide` keyword.
  
  ```plaintext
  provide $global_state to MyComponent
  ```

- **Consuming Global State**: 
  To use the global state within a component or function, use the `use` keyword.
  
  ```plaintext
  use $global_state in MyComponent
  ```

- **Local State Context**:
  Define local state within a component or function to manage component-specific state.
  
  ```plaintext
  component MyComponent {
    #local_state is { count: 0 }
    // Component logic here
  }
  ```
