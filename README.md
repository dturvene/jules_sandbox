# jules_sandbox
* https:jules.google.com using codebase git@github.com:dturvene/jules_sandbox

## stoplight.1 tag
Merge branch and tag 

```
linger$ git tag -a stoplight.1 -f -m "jules order: Write a Rust program for a stop light state machine, a synchronized crosswalk state machine and a timer task to generate events."
linger$ git push origin stoplight.1
```

* linger:350 src$ git checkout tags/stoplight.1:
```
Note: switching to 'tags/stoplight.1'.
```

### Jules command
```
Write a Rust program for a stop light state machine, a synchronized crosswalk state machine and a timer task to generate events.
```

### Build
* linger:352 src$ cargo build:
```
   Compiling stoplight_fsm v0.1.0 (/home/dturvene/ggit/jules_sandbox.git/stoplight_fsm)
warning: variant is never constructed: `StoplightIsRed`
  --> src/main.rs:80:5
   |
80 |     StoplightIsRed,
   |     ^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `StoplightIsNotRed`
  --> src/main.rs:82:5
   |
82 |     StoplightIsNotRed,
   |     ^^^^^^^^^^^^^^^^^

warning: `stoplight_fsm` (bin "stoplight_fsm") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
```

### Run
* linger:360 src$ ../target/debug/stoplight_fsm:
```
Initial Stoplight State: Red
Initial Crosswalk State: DontWalk
--- Starting simulation ---

--- Tick 0 ---
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Tick 1 ---
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Tick 2 ---
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Tick 3 ---
Stoplight is now: Red
== Pedestrian button pressed at tick 3 ==
Crosswalk button pressed.
Crosswalk changing from DontWalk to Walk
Crosswalk is now: Walk

--- Tick 4 ---
Stoplight changing from Red to Green
Stoplight is now: Green
Crosswalk is now: Walk

--- Tick 5 ---
Stoplight is now: Green
Crosswalk changing from Walk to BlinkingDontWalk
Crosswalk is now: BlinkingDontWalk

--- Tick 6 ---
Stoplight is now: Green
Crosswalk is now: BlinkingDontWalk

--- Tick 7 ---
Stoplight is now: Green
Crosswalk changing from BlinkingDontWalk to DontWalk
Crosswalk is now: DontWalk

--- Tick 8 ---
Stoplight changing from Green to Yellow
Stoplight is now: Yellow
Crosswalk is now: DontWalk

--- Tick 9 ---
Stoplight changing from Yellow to Red
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Tick 10 ---
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Tick 11 ---
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Tick 12 ---
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Tick 13 ---
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Tick 14 ---
Stoplight changing from Red to Green
Stoplight is now: Green
Crosswalk is now: DontWalk

--- Tick 15 ---
Stoplight is now: Green
Crosswalk is now: DontWalk

--- Tick 16 ---
Stoplight is now: Green
Crosswalk is now: DontWalk

--- Tick 17 ---
Stoplight is now: Green
Crosswalk is now: DontWalk

--- Tick 18 ---
Stoplight changing from Green to Yellow
Stoplight is now: Yellow
Crosswalk is now: DontWalk

--- Tick 19 ---
Stoplight changing from Yellow to Red
Stoplight is now: Red
Crosswalk is now: DontWalk

--- Simulation finished ---
```

## stoplight.2 tag
Merge branch and tag

```
linger$ git tag -a stoplight.2 -m "jules order: Write a Rust program with three threads communicating with channels.  One thread for a stop light state machine, one a synchronized crosswalk state machine and one thread to generate timer events."
linger$  git push origin stoplight.2
```

### Jules command

```
Write a Rust program with three threads communicating with channels.  One thread for a stop light state machine, one a synchronized crosswalk state machine and one thread to generate timer events.
```

### Build

* linger:364 src$ cargo build
```
   Compiling stoplight_fsm v0.1.0 (/home/dturvene/ggit/jules_sandbox.git/stoplight_fsm)
error[E0599]: no variant or associated item named `StoplightIsRed` found for enum `CrosswalkEvent` in the current scope
   --> src/main.rs:172:29
    |
98  | enum CrosswalkEvent {
    | ------------------- variant or associated item `StoplightIsRed` not found here
...
172 |             CrosswalkEvent::StoplightIsRed => {
    |                             ^^^^^^^^^^^^^^ variant or associated item not found in `CrosswalkEvent`

error[E0599]: no variant or associated item named `StoplightIsNotRed` found for enum `CrosswalkEvent` in the current scope
   --> src/main.rs:179:29
    |
98  | enum CrosswalkEvent {
    | ------------------- variant or associated item `StoplightIsNotRed` not found here
...
179 |             CrosswalkEvent::StoplightIsNotRed => {
    |                             ^^^^^^^^^^^^^^^^^ variant or associated item not found in `CrosswalkEvent`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `stoplight_fsm` due to 2 previous errors
```

## stoplight.3 tag
Comment out old CrosswalkEvent logic

### Build
* linger:395 src$ cargo build
```
   Compiling stoplight_fsm v0.1.0 (/home/dturvene/ggit/jules_sandbox.git/stoplight_fsm)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

### Run	
* linger:396 src$ ../target/debug/stoplight_fsm
```
--- Starting simulation with 25 ticks ---
Main thread listening for updates...
Timer thread: Tick 0 - Sent TimerTicks
Crosswalk thread started. Initial state: DontWalk, assuming Stoplight is Red
Stoplight thread started. Initial state: Red
Crosswalk thread: Received StoplightState: Red
Crosswalk thread: Received StoplightState: Red
Main received: Stoplight is now Red
Main received: Crosswalk is now DontWalk
Timer thread: Tick 1 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 2 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 3 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 4 - Sent TimerTicks, Simulated ButtonPress
Stoplight changing from Red to Green
Crosswalk button pressed.
Crosswalk changing from DontWalk to Walk
Crosswalk thread: Received StoplightState: Green
Crosswalk thread: Re-evaluating state due to StoplightState change from Red to Green while button_pressed_waiting_for_red is false or state was Walk.
Crosswalk changing from Walk to DontWalk because stoplight is no longer Red.
Main received: Stoplight is now Green
Main received: Crosswalk is now Walk
Main received: Crosswalk is now DontWalk
Timer thread: Tick 5 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Green
Timer thread: Tick 6 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Green
Timer thread: Tick 7 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Green
Timer thread: Tick 8 - Sent TimerTicks
Stoplight changing from Green to Yellow
Crosswalk thread: Received StoplightState: Yellow
Main received: Stoplight is now Yellow
Timer thread: Tick 9 - Sent TimerTicks, Simulated ButtonPress
Stoplight changing from Yellow to Red
Crosswalk button pressed.
Crosswalk waiting for stoplight to be Red.
Crosswalk thread: Received StoplightState: Red
Crosswalk thread: Re-evaluating state due to StoplightState change from Yellow to Red while button_pressed_waiting_for_red is true or state was DontWalk.
Crosswalk changing from DontWalk to Walk
Main received: Stoplight is now Red
Main received: Crosswalk is now Walk
Timer thread: Tick 10 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 11 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 12 - Sent TimerTicks
Crosswalk changing from Walk to BlinkingDontWalk
Crosswalk thread: Received StoplightState: Red
Main received: Crosswalk is now BlinkingDontWalk
Timer thread: Tick 13 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 14 - Sent TimerTicks, Simulated ButtonPress
Stoplight changing from Red to Green
Crosswalk changing from BlinkingDontWalk to DontWalk
Crosswalk button pressed.
Crosswalk changing from DontWalk to Walk
Crosswalk thread: Received StoplightState: Green
Crosswalk thread: Re-evaluating state due to StoplightState change from Red to Green while button_pressed_waiting_for_red is false or state was Walk.
Crosswalk changing from Walk to DontWalk because stoplight is no longer Red.
Main received: Stoplight is now Green
Main received: Crosswalk is now DontWalk
Main received: Crosswalk is now Walk
Main received: Crosswalk is now DontWalk
Timer thread: Tick 15 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Green
Timer thread: Tick 16 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Green
Timer thread: Tick 17 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Green
Timer thread: Tick 18 - Sent TimerTicks
Stoplight changing from Green to Yellow
Crosswalk thread: Received StoplightState: Yellow
Main received: Stoplight is now Yellow
Timer thread: Tick 19 - Sent TimerTicks, Simulated ButtonPress
Stoplight changing from Yellow to Red
Crosswalk button pressed.
Crosswalk waiting for stoplight to be Red.
Crosswalk thread: Received StoplightState: Red
Crosswalk thread: Re-evaluating state due to StoplightState change from Yellow to Red while button_pressed_waiting_for_red is true or state was DontWalk.
Crosswalk changing from DontWalk to Walk
Main received: Stoplight is now Red
Main received: Crosswalk is now Walk
Timer thread: Tick 20 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 21 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 22 - Sent TimerTicks
Crosswalk changing from Walk to BlinkingDontWalk
Crosswalk thread: Received StoplightState: Red
Main received: Crosswalk is now BlinkingDontWalk
Timer thread: Tick 23 - Sent TimerTicks
Crosswalk thread: Received StoplightState: Red
Timer thread: Tick 24 - Sent TimerTicks, Simulated ButtonPress
Stoplight changing from Red to Green
Crosswalk changing from BlinkingDontWalk to DontWalk
Crosswalk button pressed.
Crosswalk changing from DontWalk to Walk
Crosswalk thread: Received StoplightState: Green
Crosswalk thread: Re-evaluating state due to StoplightState change from Red to Green while button_pressed_waiting_for_red is false or state was Walk.
Crosswalk changing from Walk to DontWalk because stoplight is no longer Red.
Main received: Stoplight is now Green
Main received: Crosswalk is now DontWalk
Main received: Crosswalk is now Walk
Main received: Crosswalk is now DontWalk
Timer thread: Simulation finished. Sending shutdown signals.
Timer thread: Exiting.
Stoplight thread shutting down.
Stoplight thread terminated.
Crosswalk thread shutting down.
Crosswalk thread terminated.
Main: Stoplight FSM channel disconnected.
Main: Crosswalk FSM channel disconnected.
Main: Both FSM update channels disconnected. Exiting monitoring loop.
Main: Waiting for threads to join...
Main: Timer thread joined.
Main: Stoplight thread joined.
Main: Crosswalk thread joined.
--- Simulation finished ---
All threads joined. Main thread exiting.
```

