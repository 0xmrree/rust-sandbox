/// Global configuration for the blockchain simulator
#[derive(Debug, Clone)]
pub struct Config {
    /// Ceiling value for proof-of-work (hash must be below this)
    pub ceiling: i32,
    /// Delay after mining a block (in seconds)
    pub delay_seconds: u64,
}

impl Config {
    pub fn default() -> Self {
        Config {
            ceiling: i32::MAX, // Default: almost always mine successfully
            delay_seconds: 1,
        }
    }

    pub fn new(ceiling: i32, delay_seconds: u64) -> Self {
        Config {
            ceiling,
            delay_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.ceiling, i32::MAX);
        assert_eq!(config.delay_seconds, 1);
    }

    #[test]
    fn test_config_new() {
        let config = Config::new(1000, 5);
        assert_eq!(config.ceiling, 1000);
        assert_eq!(config.delay_seconds, 5);
    }
}
