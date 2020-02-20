mod processor;
use processor::Processor;
mod font;

fn main() {
    // Initialize graphics context
    // Initialize input handling

    // Initialize Chip-8 instance
    let mut chip8 = Processor::initialize();
    // Load a program

    let mut running = false;

    while running {
        // Run chip8 cycle

        // If chip8 drawflag is set draw graphics

        // Set chip8 keys
    }
}
