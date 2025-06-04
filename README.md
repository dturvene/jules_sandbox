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


