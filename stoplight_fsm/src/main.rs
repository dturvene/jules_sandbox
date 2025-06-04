use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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

// Messages for inter-thread communication
enum ToStoplight {
    TimerTick,
    Shutdown,
}

enum ToCrosswalk {
    TimerTick,
    ButtonPress,
    StoplightState(StoplightState), // Carries the current state of the stoplight
    Shutdown,
}

enum FromStoplight {
    StateUpdate(StoplightState), // Stoplight informs others (e.g., main loop, crosswalk) about its state
}

enum FromCrosswalk {
    StateUpdate(CrosswalkState), // Crosswalk informs others about its state
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
    // StoplightIsRed and StoplightIsNotRed are removed as per requirement.
    // This information will be conveyed via ToCrosswalk::StoplightState(StoplightState)
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
        let mut forced_by_stoplight = false; // Flag to indicate a forced transition
        match event {
            CrosswalkEvent::TimerTick => {
                self.timer_ticks_in_state += 1;
                match self.state {
                    CrosswalkState::Walk => {
                        if stoplight_state != StoplightState::Red {
                            next_state = CrosswalkState::DontWalk;
                            forced_by_stoplight = true;
                            self.button_pressed_waiting_for_red = false; // Reset waiting flag
                        } else if self.timer_ticks_in_state >= Self::WALK_DURATION {
                            next_state = CrosswalkState::BlinkingDontWalk;
                        }
                    }
                    CrosswalkState::BlinkingDontWalk => {
                        if stoplight_state != StoplightState::Red {
                            next_state = CrosswalkState::DontWalk;
                            forced_by_stoplight = true;
                            self.button_pressed_waiting_for_red = false; // Reset waiting flag
                        } else if self.timer_ticks_in_state >= Self::BLINKING_DURATION {
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

	    /*
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
	    */
        }

        if self.state != next_state {
            if forced_by_stoplight {
                println!("Crosswalk changing from {:?} to {:?} because stoplight is no longer Red.", self.state, next_state);
            } else {
                println!("Crosswalk changing from {:?} to {:?}", self.state, next_state);
            }
            self.state = next_state;
            self.timer_ticks_in_state = 0; // Reset timer for new state
        } else if event == CrosswalkEvent::TimerTick && self.state == CrosswalkState::DontWalk && self.button_pressed_waiting_for_red && stoplight_state == StoplightState::Red {
            // Special case: TimerTick while waiting for red, and light is now red
            // This transition is already covered by the main `if self.state != next_state` block if next_state was set correctly.
            // However, the original logic had this as an `else if`, implying it would only trigger if the first `if` was false.
            // Let's ensure next_state is set in DontWalk for this condition for consistency.
            // The original `DontWalk` handler already sets `next_state = CrosswalkState::Walk;`
            // and `self.button_pressed_waiting_for_red = false;` if the conditions are met.
            // The `if self.state != next_state` will then catch this.
            // So this specific `else if` might be redundant if DontWalk correctly sets next_state.
            // Let's re-verify DontWalk:
            // CrosswalkState::DontWalk => {
            //    if self.button_pressed_waiting_for_red && stoplight_state == StoplightState::Red {
            //        next_state = CrosswalkState::Walk; // This is good
            //        self.button_pressed_waiting_for_red = false;
            //    }
            // }
            // The original `else if` was likely to print a message for this specific sub-case of DontWalk.
            // Given the new `forced_by_stoplight` print, we can simplify.
            // The key is that `next_state` must be correctly set within the match block.
            // This `else if` as written would now be problematic if `next_state` was ALREADY `Walk` from the inner block.
            // Let's remove it as the main `if self.state != next_state` should handle all transitions.
            // The special case print for "TimerTick while waiting for red" is now part of the generic transition print.
        }
    }
}

// Timer thread function
fn timer_thread(
    tx_stoplight: mpsc::Sender<ToStoplight>,
    tx_crosswalk: mpsc::Sender<ToCrosswalk>,
    simulation_ticks: u32,
) {
    for tick in 0..simulation_ticks {
        let mut button_press_simulated = false;
        // Send TimerTick to Stoplight FSM
        if let Err(e) = tx_stoplight.send(ToStoplight::TimerTick) {
            eprintln!("Timer thread: failed to send TimerTick to stoplight: {}", e);
            break; // Exit loop if channel is closed
        }

        // Send TimerTick to Crosswalk FSM
        if let Err(e) = tx_crosswalk.send(ToCrosswalk::TimerTick) {
            eprintln!("Timer thread: failed to send TimerTick to crosswalk: {}", e);
            break; // Exit loop if channel is closed
        }

        // Simulate a button press every 5 ticks
        if (tick + 1) % 5 == 0 {
            if let Err(e) = tx_crosswalk.send(ToCrosswalk::ButtonPress) {
                eprintln!("Timer thread: failed to send ButtonPress to crosswalk: {}", e);
                break; // Exit loop if channel is closed
            }
            button_press_simulated = true;
        }

        if button_press_simulated {
            println!("Timer thread: Tick {} - Sent TimerTicks, Simulated ButtonPress", tick);
        } else {
            println!("Timer thread: Tick {} - Sent TimerTicks", tick);
        }

        thread::sleep(Duration::from_millis(1000)); // Sleep for 1 second per tick
    }

    println!("Timer thread: Simulation finished. Sending shutdown signals.");
    // Send Shutdown signals
    if let Err(e) = tx_stoplight.send(ToStoplight::Shutdown) {
        eprintln!("Timer thread: failed to send Shutdown to stoplight: {}", e);
    }
    if let Err(e) = tx_crosswalk.send(ToCrosswalk::Shutdown) {
        eprintln!("Timer thread: failed to send Shutdown to crosswalk: {}", e);
    }
    println!("Timer thread: Exiting.");
}

// Stoplight thread function
fn stoplight_thread(
    rx: mpsc::Receiver<ToStoplight>,
    tx_main: Option<mpsc::Sender<FromStoplight>>,
    tx_crosswalk: Option<mpsc::Sender<ToCrosswalk>>,
) {
    let mut fsm = StoplightFsm::new();
    println!("Stoplight thread started. Initial state: {:?}", fsm.state);

    // Send initial state to main (if channel provided)
    if let Some(ref sender) = tx_main {
        if let Err(e) = sender.send(FromStoplight::StateUpdate(fsm.state)) {
            eprintln!("Stoplight thread: failed to send initial state to main: {}", e);
        }
    }

    // Send initial state to crosswalk (if channel provided)
    if let Some(ref sender) = tx_crosswalk {
        if let Err(e) = sender.send(ToCrosswalk::StoplightState(fsm.state)) {
            eprintln!("Stoplight thread: failed to send initial state to crosswalk: {}", e);
        }
    }

    while let Ok(message) = rx.recv() {
        match message {
            ToStoplight::TimerTick => {
                let old_state = fsm.state;
                fsm.handle_event(StoplightEvent::TimerTick);
                // The println inside handle_event already announces the change,
                // but we can add a specific one for the thread context if needed.
                // println!("Stoplight thread: Processed TimerTick. Current state: {:?}", fsm.state);

                // If state changed, send update to main
                if let Some(ref sender) = tx_main {
                    if old_state != fsm.state { // Send only if state changed
                        if let Err(e) = sender.send(FromStoplight::StateUpdate(fsm.state)) {
                            eprintln!("Stoplight thread: failed to send state update to main: {}", e);
                        }
                    }
                }
                // Always send current state to crosswalk thread
                if let Some(ref sender) = tx_crosswalk {
                    if let Err(e) = sender.send(ToCrosswalk::StoplightState(fsm.state)) {
                        eprintln!("Stoplight thread: failed to send state to crosswalk: {}", e);
                        // If crosswalk channel is broken, we might not need to shut down stoplight,
                        // but it's a sign something is wrong.
                    }
                }
            }
            ToStoplight::Shutdown => {
                println!("Stoplight thread shutting down.");
                break;
            }
        }
    }
    println!("Stoplight thread terminated.");
}

// Crosswalk thread function
fn crosswalk_thread(
    rx: mpsc::Receiver<ToCrosswalk>,
    tx_main: Option<mpsc::Sender<FromCrosswalk>>,
) {
    let mut fsm = CrosswalkFsm::new();
    // Default to Red, will be updated by the first message from stoplight_thread
    let mut current_stoplight_state = StoplightState::Red;
    println!("Crosswalk thread started. Initial state: {:?}, assuming Stoplight is {:?}", fsm.state, current_stoplight_state);

    // Send initial state to main (if channel provided)
    if let Some(ref sender) = tx_main {
        if let Err(e) = sender.send(FromCrosswalk::StateUpdate(fsm.state)) {
            eprintln!("Crosswalk thread: failed to send initial state to main: {}", e);
        }
    }

    while let Ok(message) = rx.recv() {
        let old_fsm_state = fsm.state;
        match message {
            ToCrosswalk::TimerTick => {
                // println!("Crosswalk thread: Received TimerTick. Current stoplight state: {:?}", current_stoplight_state);
                fsm.handle_event(CrosswalkEvent::TimerTick, current_stoplight_state);
            }
            ToCrosswalk::ButtonPress => {
                // println!("Crosswalk thread: Received ButtonPress. Current stoplight state: {:?}", current_stoplight_state);
                // The CrosswalkFsm::handle_event for ButtonPress already prints "Crosswalk button pressed."
                fsm.handle_event(CrosswalkEvent::ButtonPress, current_stoplight_state);
            }
            ToCrosswalk::StoplightState(new_state) => {
                println!("Crosswalk thread: Received StoplightState: {:?}", new_state);
                let old_stoplight_state = current_stoplight_state;
                current_stoplight_state = new_state;

                // If the crosswalk was waiting for a red light, and the light is now red,
                // or if the light is no longer red and it was walking/blinking.
                // We should re-evaluate its state.
                // The CrosswalkFsm's TimerTick event is a good way to do this,
                // as it checks button_pressed_waiting_for_red and current stoplight state.
                // Also, handle cases where stoplight changes from Red to something else, potentially forcing DontWalk.
                if (fsm.button_pressed_waiting_for_red && current_stoplight_state == StoplightState::Red) ||
                   (old_stoplight_state == StoplightState::Red && current_stoplight_state != StoplightState::Red && (fsm.state == CrosswalkState::Walk || fsm.state == CrosswalkState::BlinkingDontWalk))
                {
                    // This print helps understand the re-evaluation trigger
                    println!("Crosswalk thread: Re-evaluating state due to StoplightState change from {:?} to {:?} while button_pressed_waiting_for_red is {} or state was {:?}.", old_stoplight_state, current_stoplight_state, fsm.button_pressed_waiting_for_red, fsm.state);
                    fsm.handle_event(CrosswalkEvent::TimerTick, current_stoplight_state);
                }
            }
            ToCrosswalk::Shutdown => {
                println!("Crosswalk thread shutting down.");
                break;
            }
        }

        // If FSM state changed, send update to main
        if old_fsm_state != fsm.state {
            if let Some(ref sender) = tx_main {
                if let Err(e) = sender.send(FromCrosswalk::StateUpdate(fsm.state)) {
                    eprintln!("Crosswalk thread: failed to send state update to main: {}", e);
                }
            }
        }
    }
    println!("Crosswalk thread terminated.");
}

fn main() {
    const SIMULATION_TICKS: u32 = 25;

    // Create channels
    let (tx_to_stoplight, rx_from_timer_for_stoplight) = mpsc::channel::<ToStoplight>();
    let (tx_to_crosswalk_combined, rx_for_crosswalk_combined) = mpsc::channel::<ToCrosswalk>();
    let (tx_from_stoplight_to_main, rx_from_stoplight_for_main) = mpsc::channel::<FromStoplight>();
    let (tx_from_crosswalk_to_main, rx_from_crosswalk_for_main) = mpsc::channel::<FromCrosswalk>();

    // Clone sender for crosswalk as it's used by timer and stoplight threads
    let tx_to_crosswalk_for_timer = tx_to_crosswalk_combined.clone();
    // The last sender tx_to_crosswalk_combined can be moved directly to the stoplight thread
    let tx_to_crosswalk_for_stoplight = tx_to_crosswalk_combined;

    println!("--- Starting simulation with {} ticks ---", SIMULATION_TICKS);

    // Spawn Timer Thread
    let timer_handle = thread::spawn(move || {
        timer_thread(tx_to_stoplight, tx_to_crosswalk_for_timer, SIMULATION_TICKS);
    });

    // Spawn Stoplight Thread
    let stoplight_handle = thread::spawn(move || {
        stoplight_thread(
            rx_from_timer_for_stoplight,
            Some(tx_from_stoplight_to_main),
            Some(tx_to_crosswalk_for_stoplight),
        );
    });

    // Spawn Crosswalk Thread
    let crosswalk_handle = thread::spawn(move || {
        crosswalk_thread(rx_for_crosswalk_combined, Some(tx_from_crosswalk_to_main));
    });

    // Main Monitoring Loop
    println!("Main thread listening for updates...");
    let mut stoplight_updates_active = true;
    let mut crosswalk_updates_active = true;

    loop {
        if !stoplight_updates_active && !crosswalk_updates_active {
            println!("Main: Both FSM update channels disconnected. Exiting monitoring loop.");
            break;
        }

        // Check for messages from Stoplight FSM
        if stoplight_updates_active {
            match rx_from_stoplight_for_main.try_recv() {
                Ok(FromStoplight::StateUpdate(state)) => {
                    println!("Main received: Stoplight is now {:?}", state);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // No message currently available
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Main: Stoplight FSM channel disconnected.");
                    stoplight_updates_active = false;
                }
            }
        }

        // Check for messages from Crosswalk FSM
        if crosswalk_updates_active {
            match rx_from_crosswalk_for_main.try_recv() {
                Ok(FromCrosswalk::StateUpdate(state)) => {
                    println!("Main received: Crosswalk is now {:?}", state);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // No message currently available
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Main: Crosswalk FSM channel disconnected.");
                    crosswalk_updates_active = false;
                }
            }
        }

        // Avoid busy-waiting if both channels are still active but empty
        if stoplight_updates_active || crosswalk_updates_active {
            thread::sleep(Duration::from_millis(50)); // Short sleep to yield CPU
        }
    }

    // Join Threads
    println!("Main: Waiting for threads to join...");
    timer_handle.join().expect("Timer thread panicked");
    println!("Main: Timer thread joined.");
    stoplight_handle.join().expect("Stoplight thread panicked");
    println!("Main: Stoplight thread joined.");
    crosswalk_handle.join().expect("Crosswalk thread panicked");
    println!("Main: Crosswalk thread joined.");

    println!("--- Simulation finished ---");
    println!("All threads joined. Main thread exiting.");
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

    #[test]
    fn test_crosswalk_cycle_prematurely_ends_if_light_changes() {
        // Test Walk to DontWalk if light changes from Red
        let mut fsm_walk_test = CrosswalkFsm::new();
        // 1. Get to Walk state
        fsm_walk_test.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Red);
        assert_eq!(fsm_walk_test.state, CrosswalkState::Walk, "Test Walk: Should be in Walk state after button press with Red light");
        assert_eq!(fsm_walk_test.timer_ticks_in_state, 0, "Test Walk: Timer should be 0 after transitioning to Walk");

        // 2. Simulate one TimerTick with StoplightState::Red (assuming WALK_DURATION > 1)
        // Ensure WALK_DURATION is suitable for this test logic. If WALK_DURATION is 1, this step might transition state.
        if CrosswalkFsm::WALK_DURATION > 1 {
            fsm_walk_test.handle_event(CrosswalkEvent::TimerTick, StoplightState::Red);
            assert_eq!(fsm_walk_test.state, CrosswalkState::Walk, "Test Walk: Should still be in Walk state after 1 tick if duration > 1");
            assert_eq!(fsm_walk_test.timer_ticks_in_state, 1, "Test Walk: Timer should be 1");
        } else {
            // If WALK_DURATION is 1, after one tick it will be BlinkingDontWalk.
            // This part of the test needs adjustment if WALK_DURATION is 1.
            // For now, proceeding with assumption WALK_DURATION > 1 based on typical values (e.g., 3).
            // If it transitions, the next step's premise is wrong.
            println!("Note: WALK_DURATION is 1, Walk state will transition immediately to BlinkingDontWalk after 1 tick.");
        }


        // 3. Simulate a TimerTick with StoplightState::Green. Should transition to DontWalk.
        fsm_walk_test.handle_event(CrosswalkEvent::TimerTick, StoplightState::Green);
        assert_eq!(fsm_walk_test.state, CrosswalkState::DontWalk, "Test Walk: Should transition to DontWalk if light turns Green");
        assert_eq!(fsm_walk_test.timer_ticks_in_state, 0, "Test Walk: Timer should reset after forced transition to DontWalk");

        // Test BlinkingDontWalk to DontWalk if light changes from Red
        let mut fsm_blink_test = CrosswalkFsm::new();
        // 1. Get to Walk state first
        fsm_blink_test.handle_event(CrosswalkEvent::ButtonPress, StoplightState::Red);
        assert_eq!(fsm_blink_test.state, CrosswalkState::Walk, "Test Blink: Initial transition to Walk failed");

        // 2. Tick through Walk state to reach BlinkingDontWalk
        for _ in 0..CrosswalkFsm::WALK_DURATION {
            fsm_blink_test.handle_event(CrosswalkEvent::TimerTick, StoplightState::Red);
        }
        assert_eq!(fsm_blink_test.state, CrosswalkState::BlinkingDontWalk, "Test Blink: Should be in BlinkingDontWalk after WALK_DURATION ticks");
        assert_eq!(fsm_blink_test.timer_ticks_in_state, 0, "Test Blink: Timer should reset after transitioning to BlinkingDontWalk");

        // 3. Simulate one TimerTick with StoplightState::Red (assuming BLINKING_DURATION > 1)
        if CrosswalkFsm::BLINKING_DURATION > 1 {
            fsm_blink_test.handle_event(CrosswalkEvent::TimerTick, StoplightState::Red);
            assert_eq!(fsm_blink_test.state, CrosswalkState::BlinkingDontWalk, "Test Blink: Should still be in BlinkingDontWalk after 1 tick if duration > 1");
            assert_eq!(fsm_blink_test.timer_ticks_in_state, 1, "Test Blink: Timer should be 1 for BlinkingDontWalk");
        } else {
            println!("Note: BLINKING_DURATION is 1, BlinkingDontWalk will transition immediately to DontWalk after 1 tick.");
        }


        // 4. Simulate a TimerTick with StoplightState::Green. Should transition to DontWalk.
        fsm_blink_test.handle_event(CrosswalkEvent::TimerTick, StoplightState::Green);
        assert_eq!(fsm_blink_test.state, CrosswalkState::DontWalk, "Test Blink: Should transition to DontWalk if light turns Green during BlinkingDontWalk");
        assert_eq!(fsm_blink_test.timer_ticks_in_state, 0, "Test Blink: Timer should reset after forced transition to DontWalk from BlinkingDontWalk");
    }
}
