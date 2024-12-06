# Why Rust Complains About Unused Variables

The Rust compiler raises warnings (or errors, depending on the settings) for unused variables because Rust is designed with a focus on safety and efficiency. An unused variable might indicate a logic error or unnecessary resource allocation, which the compiler encourages you to address.

* Potential Bugs: An unused variable might indicate a mistake in the code where a value is computed but never used. Highlighting this helps developers catch bugs early.
* Code Clarity: Removing unused variables makes the code cleaner and easier to maintain.
* Efficiency: Unused variables could lead to unnecessary memory usage or computation.