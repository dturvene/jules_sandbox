use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone, Copy)]
enum StoplightState {
    Red,
    Green,
    Yellow,
}

#[derive(Debug, PartialEq)]
enum StoplightEvent {
    TimerTick,
}

struct StoplightFsm {
    state: StoplightState,
    timer_ticks_in_state: u32, // Counter for how long we've been in the current state
}

impl StoplightFsm {
    fn new() -> Self {
        StoplightFsm {
            state: StoplightState::Red, // Initial state
            timer_ticks_in_state: 0,
        }
    }

    // Constants for state durations (in TimerTicks)
    const RED_DURATION: u32 = 5;
    const GREEN_DURATION: u32 = 4;
    const YELLOW_DURATION: u32 = 1;

    fn handle_event(&mut self, event: StoplightEvent) {
        match event {
            StoplightEvent::TimerTick => {
                self.timer_ticks_in_state += 1;
                let mut next_state = self.state; // Default to current state

                match self.state {
                    StoplightState::Red => {
                        if self.timer_ticks_in_state >= Self::RED_DURATION {
                            next_state = StoplightState::Green;
                        }
                    }
                    StoplightState::Green => {
                        if self.timer_ticks_in_state >= Self::GREEN_DURATION {
                            next_state = StoplightState::Yellow;
                        }
                    }
                    StoplightState::Yellow => {
                        if self.timer_ticks_in_state >= Self::YELLOW_DURATION {
                            next_state = StoplightState::Red;
                        }
                    }
                }

                if self.state != next_state {
                    println!("Stoplight changing from {:?} to {:?}", self.state, next_state);
                    self.state = next_state;
                    self.timer_ticks_in_state = 0; // Reset timer for new state
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum CrosswalkState {
    DontWalk,
    Walk,
    BlinkingDontWalk,
}

#[derive(Debug, PartialEq)]
enum CrosswalkEvent {
    TimerTick,
    ButtonPress,
    // Event from stoplight to signal it's safe to walk
    StoplightIsRed,
    // Event from stoplight to signal it's no longer safe
    StoplightIsNotRed,
}

struct CrosswalkFsm {
    state: CrosswalkState,
    timer_ticks_in_state: u32,
    button_pressed_waiting_for_red: bool, // Flag to remember if button was pressed
}

impl CrosswalkFsm {
    fn new() -> Self {
        CrosswalkFsm {
            state: CrosswalkState::DontWalk,
            timer_ticks_in_state: 0,
            button_pressed_waiting_for_red: false,
        }
    }

    // Constants for state durations
    const WALK_DURATION: u32 = 3; // How long "Walk" stays on
    const BLINKING_DURATION: u32 = 2; // How long "DontWalk" blinks

    fn handle_event(&mut self, event: CrosswalkEvent, stoplight_state: StoplightState) {
        let mut next_state = self.state;
        match event {
            CrosswalkEvent::TimerTick => {
                self.timer_ticks_in_state += 1;
                match self.state {
                    CrosswalkState::Walk => {
                        if self.timer_ticks_in_state >= Self::WALK_DURATION {
                            next_state = CrosswalkState::BlinkingDontWalk;
                        }
                    }
                    CrosswalkState::BlinkingDontWalk => {
                        if self.timer_ticks_in_state >= Self::BLINKING_DURATION {
                            next_state = CrosswalkState::DontWalk;
                        }
                    }
                    CrosswalkState::DontWalk => {
                        // If button was pressed and light is now Red, transition to Walk
                        if self.button_pressed_waiting_for_red && stoplight_state == StoplightState::Red {
                            next_state = CrosswalkState::Walk;
                            self.button_pressed_waiting_for_red = false; // Reset flag
                        }
                    }
                }
            }
            CrosswalkEvent::ButtonPress => {
                println!("Crosswalk button pressed.");
                if self.state == CrosswalkState::DontWalk {
                    if stoplight_state == StoplightState::Red {
                        // If light is already red, transition immediately
                        next_state = CrosswalkState::Walk;
                    } else {
                        // Otherwise, set flag and wait for StoplightIsRed event (or for timer tick when light is red)
                        self.button_pressed_waiting_for_red = true;
                        println!("Crosswalk waiting for stoplight to be Red.");
                    }
                }
                // If already Walk or Blinking, button press is ignored or could reset timer (not implemented here)
            }
            CrosswalkEvent::StoplightIsRed => {
                // This event is triggered by the main loop when stoplight turns red
                if self.state == CrosswalkState::DontWalk && self.button_pressed_waiting_for_red {
                    next_state = CrosswalkState::Walk;
                    self.button_pressed_waiting_for_red = false;
                }
            }
            CrosswalkEvent::StoplightIsNotRed => {
                // If stoplight is no longer red, ensure crosswalk is not in Walk state
                if self.state == CrosswalkState::Walk || self.state == CrosswalkState::BlinkingDontWalk {
                    next_state = CrosswalkState::DontWalk;
                    println!("Stoplight no longer red, forcing Crosswalk to DontWalk.");
                }
                 self.button_pressed_waiting_for_red = false; // Cancel any pending walk request
            }
        }

        if self.state != next_state {
            println!("Crosswalk changing from {:?} to {:?}", self.state, next_state);
            self.state = next_state;
            self.timer_ticks_in_state = 0; // Reset timer for new state
        } else if event == CrosswalkEvent::TimerTick && self.state == CrosswalkState::DontWalk && self.button_pressed_waiting_for_red && stoplight_state == StoplightState::Red {
            // Special case: TimerTick while waiting for red, and light is now red
             println!("Crosswalk changing from {:?} to {:?}", self.state, CrosswalkState::Walk);
            self.state = CrosswalkState::Walk;
            self.timer_ticks_in_state = 0;
            self.button_pressed_waiting_for_red = false;
        }
    }
}

fn main() {
    let mut stoplight = StoplightFsm::new();
    let mut crosswalk = CrosswalkFsm::new();

    println!("Initial Stoplight State: {:?}", stoplight.state);
    println!("Initial Crosswalk State: {:?}", crosswalk.state);
    println!("--- Starting simulation ---");

    // Simulate a button press for the crosswalk after a few ticks
    let crosswalk_button_press_tick = 3;

    for tick in 0..20 { // Simulate 20 ticks
        println!("\n--- Tick {} ---", tick);

        // 1. Handle Stoplight Event
        stoplight.handle_event(StoplightEvent::TimerTick);
        println!("Stoplight is now: {:?}", stoplight.state);

        // 2. Check for crosswalk button press at specific tick
        if tick == crosswalk_button_press_tick {
            println!("== Pedestrian button pressed at tick {} ==", tick);
            crosswalk.handle_event(CrosswalkEvent::ButtonPress, stoplight.state);
        }

        // 3. Determine if stoplight state change affects crosswalk
        // This is a simplified way to send an event; a more robust system might use channels or callbacks.
        // We need to capture the previous stoplight state to detect a change to Red or from Red.
        let _previous_stoplight_state_for_crosswalk_logic = stoplight.state; // Assume it was handled this tick.
                                                                      // A more accurate way would be to store state *before* handle_event

        // If stoplight just turned Red, inform crosswalk
        // This logic is a bit tricky because stoplight.handle_event might have changed its state.
        // For simplicity, we'll use the current stoplight.state. A better approach would be to
        // get the state *before* stoplight.handle_event and *after* to see if it turned red *this tick*.
        // Or, the stoplight FSM itself could emit an event "TurnedRed".
        // For now, the crosswalk's TimerTick handler also checks if it's waiting and light is red.

        // 4. Handle Crosswalk Event (TimerTick)
        // We also need to inform the crosswalk if the light is red or not for its internal logic.
        let current_stoplight_state = stoplight.state; // Get current state after stoplight processed its tick

        // If the stoplight just turned red this tick (which we infer by checking its current state,
        // assuming it was not red before its own TimerTick handling if it changed)
        // OR if the stoplight is no longer red
        // This part is tricky. The crosswalk `handle_event` for TimerTick already has logic
        // to transition to Walk if button_pressed_waiting_for_red is true and stoplight is Red.
        // Let's refine the crosswalk events.
        // Instead of StoplightIsRed/StoplightIsNotRed events triggered from main loop like this,
        // the crosswalk's TimerTick and ButtonPress handlers will directly use the passed `stoplight.state`.
        // The StoplightIsRed / StoplightIsNotRed events in CrosswalkEvent enum can be removed or re-purposed
        // if we had a more direct communication from StoplightFSM.
        // For now, the existing crosswalk handle_event logic which takes stoplight_state as a parameter is sufficient.

        crosswalk.handle_event(CrosswalkEvent::TimerTick, current_stoplight_state);
        println!("Crosswalk is now: {:?}", crosswalk.state);


        // Simulate time passing
        thread::sleep(Duration::from_millis(500)); // Sleep for 0.5 seconds per tick
    }
    println!("\n--- Simulation finished ---");
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope (main.rs)

    #[test]
    fn test_stoplight_cycle() {
        let mut fsm = StoplightFsm::new();
        assert_eq!(fsm.state, StoplightState::Red);

        // Tick through Red state
        for _ in 0..StoplightFsm::RED_DURATION {
            fsm.handle_event(StoplightEvent::TimerTick);
        }
        assert_eq!(fsm.state, StoplightState::Green);
        assert_eq!(fsm.timer_ticks_in_state, 0); // Timer should reset

        // Tick through Green state
        for _ in 0..StoplightFsm::GREEN_DURATION {
            fsm.handle_event(StoplightEvent::TimerTick);
        }
        assert_eq!(fsm.state, StoplightState::Yellow);
        assert_eq!(fsm.timer_ticks_in_state, 0);

        // Tick through Yellow state
        for _ in 0..StoplightFsm::YELLOW_DURATION {
            fsm.handle_event(StoplightEvent::TimerTick);
        }
        assert_eq!(fsm.state, StoplightState::Red);
        assert_eq!(fsm.timer_ticks_in_state, 0);
    }

    #[test]
    fn test_crosswalk_button_press_when_stoplight_red() {
        let mut crosswalk_fsm = CrosswalkFsm::new();
        assert_eq!(crosswalk_fsm.state, CrosswalkState::DontWalk);

        // Simulate button press when stoplight is Red
        crosswalk_fsm.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Red);
        assert_eq!(crosswalk_fsm.state, CrosswalkState::Walk);
        assert_eq!(crosswalk_fsm.timer_ticks_in_state, 0);

        // Tick through Walk state
        for _ in 0..CrosswalkFsm::WALK_DURATION {
            crosswalk_fsm.handle_event(CrosswalkEvent::TimerTick, StoplightState::Red);
        }
        assert_eq!(crosswalk_fsm.state, CrosswalkState::BlinkingDontWalk);
        assert_eq!(crosswalk_fsm.timer_ticks_in_state, 0);

        // Tick through BlinkingDontWalk state
        for _ in 0..CrosswalkFsm::BLINKING_DURATION {
            crosswalk_fsm.handle_event(CrosswalkEvent::TimerTick, StoplightState::Red);
        }
        assert_eq!(crosswalk_fsm.state, CrosswalkState::DontWalk);
        assert_eq!(crosswalk_fsm.timer_ticks_in_state, 0);
    }

    #[test]
    fn test_crosswalk_button_press_when_stoplight_green_then_red() {
        let mut crosswalk_fsm = CrosswalkFsm::new();
        assert_eq!(crosswalk_fsm.state, CrosswalkState::DontWalk);

        // Simulate button press when stoplight is Green
        crosswalk_fsm.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Green);
        assert_eq!(crosswalk_fsm.state, CrosswalkState::DontWalk); // Should not change yet
        assert!(crosswalk_fsm.button_pressed_waiting_for_red);

        // Simulate some time passing, stoplight still Green
        crosswalk_fsm.handle_event(CrosswalkEvent::TimerTick, StoplightState::Green);
        assert_eq!(crosswalk_fsm.state, CrosswalkState::DontWalk);
        assert!(crosswalk_fsm.button_pressed_waiting_for_red);

        // Simulate stoplight turning Red (via TimerTick to crosswalk while stoplight is Red)
        crosswalk_fsm.handle_event(CrosswalkEvent::TimerTick, StoplightState::Red);
        assert_eq!(crosswalk_fsm.state, CrosswalkState::Walk); // Now it should change
        assert!(!crosswalk_fsm.button_pressed_waiting_for_red);
        assert_eq!(crosswalk_fsm.timer_ticks_in_state, 0);
    }

    #[test]
    fn test_crosswalk_forced_to_dont_walk_if_stoplight_not_red() {
        let mut crosswalk_fsm = CrosswalkFsm::new();
        // Make it Walk
        crosswalk_fsm.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Red);
        assert_eq!(crosswalk_fsm.state, CrosswalkState::Walk);

        // Simulate stoplight turning Green. Crosswalk should be forced to DontWalk.
        // In our current main loop, this is implicitly handled by passing the new stoplight state
        // to the crosswalk's TimerTick.
        // We can also use the (currently unused by main loop) StoplightIsNotRed event for a more direct test.
        crosswalk_fsm.handle_event(CrosswalkEvent::StoplightIsNotRed, StoplightState::Green);
        assert_eq!(crosswalk_fsm.state, CrosswalkState::DontWalk);
        assert_eq!(crosswalk_fsm.timer_ticks_in_state, 0);

        // Test with BlinkingDontWalk state
        crosswalk_fsm.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Red); // Go to Walk
        for _ in 0..CrosswalkFsm::WALK_DURATION { // Go to Blinking
            crosswalk_fsm.handle_event(CrosswalkEvent::TimerTick, StoplightState::Red);
        }
        assert_eq!(crosswalk_fsm.state, CrosswalkState::BlinkingDontWalk);
        crosswalk_fsm.handle_event(CrosswalkEvent::StoplightIsNotRed, StoplightState::Green);
        assert_eq!(crosswalk_fsm.state, CrosswalkState::DontWalk);
        assert_eq!(crosswalk_fsm.timer_ticks_in_state, 0);
    }

    #[test]
    fn test_crosswalk_button_press_ignored_if_not_dont_walk() {
        let mut crosswalk_fsm = CrosswalkFsm::new();
        crosswalk_fsm.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Red); // -> Walk
        assert_eq!(crosswalk_fsm.state, CrosswalkState::Walk);
        let current_ticks = crosswalk_fsm.timer_ticks_in_state;

        crosswalk_fsm.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Red); // Press button again
        assert_eq!(crosswalk_fsm.state, CrosswalkState::Walk); // State should not change
        assert_eq!(crosswalk_fsm.timer_ticks_in_state, current_ticks); // Ticks should not reset

        for _ in 0..CrosswalkFsm::WALK_DURATION {
             crosswalk_fsm.handle_event(CrosswalkEvent::TimerTick, StoplightState::Red);
        }
        assert_eq!(crosswalk_fsm.state, CrosswalkState::BlinkingDontWalk); // -> BlinkingDontWalk
        let current_ticks_blinking = crosswalk_fsm.timer_ticks_in_state;

        crosswalk_fsm.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Red); // Press button again
        assert_eq!(crosswalk_fsm.state, CrosswalkState::BlinkingDontWalk); // State should not change
        assert_eq!(crosswalk_fsm.timer_ticks_in_state, current_ticks_blinking); // Ticks should not reset
    }
}
