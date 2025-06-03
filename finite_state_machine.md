# Finite State Machine (FSM)

A **Finite State Machine (FSM)** or **Finite State Automaton (FSA)** is a mathematical model of computation. It is an abstract machine that can be in exactly one of a finite number of *states* at any given time.

The FSM can change from one state to another in response to some *inputs*; the change from one state to another is called a *transition*.

An FSM is defined by:
*   A list of its **states**.
*   Its **initial state**.
*   The **inputs** that trigger each transition.

## Types of Finite State Machines

Finite-state machines are of two types:
*   **Deterministic Finite State Machines (DFSM):** In a DFSM, for each state, there is exactly one transition for each possible input.
*   **Non-deterministic Finite State Machines (NFSM):** In an NFSM, for each state, an input can lead to one, more than one, or no transition. It's worth noting that for any non-deterministic finite-state machine, an equivalent deterministic one can be constructed.

## Example: Coin-Operated Turnstile

A simple example of an FSM is a coin-operated turnstile.

*   **States:**
    *   `Locked`: The turnstile is locked, preventing entry.
    *   `Unlocked`: The turnstile is unlocked, allowing one entry.
*   **Inputs:**
    *   `coin`: A coin is inserted into the turnstile.
    *   `push`: A person pushes the arm of the turnstile.
*   **Transitions:**
    *   If the state is `Locked` and the input is `coin`, the state changes to `Unlocked`.
    *   If the state is `Locked` and the input is `push`, the state remains `Locked`.
    *   If the state is `Unlocked` and the input is `coin`, the state remains `Unlocked` (further coins do not change the state until it's re-locked).
    *   If the state is `Unlocked` and the input is `push`, the state changes to `Locked` (after allowing entry).

### State Diagram

```mermaid
graph TD
    [*] --> Locked;
    Locked -- coin --> Unlocked;
    Locked -- push --> Locked;
    Unlocked -- push --> Locked;
    Unlocked -- coin --> Unlocked;
```
*(Note: This uses Mermaid syntax for the diagram, which will render in many Markdown viewers.)*

## Common Representations of FSMs

FSMs can be represented in several ways:

*   **State/Event Table (or State Transition Table):** This table shows the next state and output for each combination of current state and input.

    | Current State | Input | Next State | Output                        |
    |---------------|-------|------------|-------------------------------|
    | Locked        | coin  | Unlocked   | Unlocks turnstile             |
    | Locked        | push  | Locked     | None                          |
    | Unlocked      | coin  | Unlocked   | None                          |
    | Unlocked      | push  | Locked     | Locks turnstile (after entry) |

*   **State Diagram:** A directed graph where nodes represent states and edges (arrows) represent transitions. Each arrow is labeled with the input that triggers the transition. The initial state is usually marked with an arrow coming from a filled black dot. Accepting states (if applicable) are often marked with a double circle.

## Common Usages of FSMs

Finite State Machines are used in many devices and systems, including:

*   Vending machines
*   Elevators
*   Traffic lights
*   Combination locks
*   Lexical analysis and parsing in compilers
*   Network protocols
*   Video game character AI
*   Control systems in robotics
