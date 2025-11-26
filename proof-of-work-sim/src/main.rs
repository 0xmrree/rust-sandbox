// Module declarations
mod config;
mod traits;
mod transaction;
mod block;
mod blockchain;
mod node;

// Re-exports for convenience
use config::Config;
use node::Node;

fn main() {
    println!("⛏️  Proof-of-Work Blockchain Simulator\n");
    
    // Create configuration
    let config = Config::default();
    
    println!("⚙️  Configuration:");
    println!("   Ceiling: {} ({})", config.ceiling, 
             if config.ceiling == i32::MAX { "almost always mines" } else { "challenging" });
    println!("   Delay: {} second(s)\n", config.delay_seconds);
    
    // Create and start a mining node
    let mut node = Node::new(config);
    node.start_mining();
}
