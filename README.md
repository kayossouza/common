
# Common Language (WIP)

## 🚀 Overview

**Common** is a new programming language that aims to bring simplicity and clarity by adopting a conversational, human-friendly syntax. The goal is to create a language that feels as natural as writing everyday sentences while being powerful enough for modern application development. Think of it as chatting with your computer, but with code!

The ultimate vision for **Common** is to be **the only language you'll ever need**, capable of generating **web**, **mobile**, **embedded**, and **server** applications—essentially, everything.

This language is still a work-in-progress, so expect the unexpected (both bugs and delightful surprises)! 🤖

## 🎯 Vision

The **Common** language is being built with the following principles in mind:

- **Readability**: Code should read like English. `age is 25`? You got it.
- **Ease of use**: Creating variables, functions, and loops should feel intuitive, even for beginners.
- **Flexibility**: We support dynamic typing, state management (yes, global and local, we’re not playing around), and a simplified flow of logical and conditional statements.
- **Fun**: Why should coding be dull? This language has a few tricks up its sleeve—expect easter eggs, jokes, and surprises along the way. 😎

## ✨ Features (so far...)

### 1. Variable Declarations
Declare a variable like you're having a conversation:
```common
age is 25
username is "JohnDoe"
```

### 2. Display Output
Want to print something? Just ask:
```common
display "Hello, World!"
```

### 3. Conditional Statements
When logic is simple, life is good:
```common
when age is greater than 18
  display "You are an adult"
else
  display "You are a minor"
```

### 4. Loops
Don’t overthink it, just tell the code what to do:
```common
do this 5 times
  display "This is looped!"
end
```

### 5. State Management
Because every serious language has global and local state, right?
```common
$global_state is { logged_in: false }
#local_state is { user: "Guest" }
```

### 🌱 What's Coming?

- **Function Declarations**: They’ll be here soon, and yes, you’ll declare them like: `create a function called`.
- **Advanced State Handling**: State management will feel like managing your RPG inventory but with fewer potions. 🧙‍♂️
- **Complex Conditionals and Loops**: Because we know you want more control.

## 🛠️ Development

This project is in its early days, and we’ve got some rough edges. There’s a lot to build, but hey, Rome wasn’t built in a day (and it definitely didn’t have this many bugs).

### 🐛 Known Issues

- **State management**: It currently works, but it's like an unreliable sidekick—it does what it wants sometimes.
- **Function calls**: Halfway implemented, but let’s just say they need some... TLC.
- **Error Handling**: If you write something weird, don’t expect a friendly message... yet.

## 🧑‍💻 How to Run It

1. Clone this repo.
2. Build it with Cargo:
   ```bash
   cargo build
   ```
3. Run the project:
   ```bash
   cargo run
   ```
4. Enter your conversational code, like `display "Hello, World!"` and watch the magic happen.

## 🎉 Easter Eggs

- If you try typing `create a function called MrPickles`, the program will ask if you’re sure you want to summon "the god of computer science." 🐕🐾
- The `do this 5 times` command is secretly counting every loop run, and if you ever reach 9999 iterations, let’s just say, the compiler might “go rogue.” 🤖
- Stay tuned for more fun in the next release. You never know what you might find. 

## 🛡️ Disclaimer

Common is a work in progress, so please don’t use it to run a nuclear reactor or control your smart fridge… yet. 💥

## 👏 Contributions

Feel free to fork, star, or open issues. Contributions, especially weirdly named functions, are highly encouraged. 😉

## 👀 Stay Tuned

This is just the beginning! Follow the repo and stay updated as we move from conversational coding to world domination... one `display` at a time. 🌎

---

*P.S.: If you find any bugs, report them. If the bug ends up being a feature, congratulations, you’ve contributed to the chaos!* 🐞🎉
