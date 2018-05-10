# Attic

Solutions to [hackattic](http://hackattic.com/) challenges. 

## Organization

The base crate contains specific low-level implementations needed to run the challenges via a CLI app. 

Most of the application logic is contained in the **core-lib** package. Its source code is organized into several root modules:

- **toolbox** provides the functions used to solve individual challenges (center of the onion)
- **solution** contains the solution implementations for specific challenges (use cases)
- **challenge**
  - **solver** contains solution-specific setup that implement the `SolvesChallenge` trait. These are responsible for parsing problem data and formatting the solution.  
  - `ChallengeGateway` is the trait used to get challenge problems and evaluate the solutions

A `solver` and a `solution` are needed for each solution's implementation. General-purpose code used in the `solution` should go in the **toolbox** as much as possible. 

## Testing

The base crate only contains low-level adapters. Testing is done by manually running the application.

In the **core-lib** crate, high-level modules (**toolbox** & **solution**) contain unit tests. The **tests** directory contains outside-in integration tests.
