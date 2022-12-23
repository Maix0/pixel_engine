#[derive(Debug)]
struct Uncreatable;

impl std::fmt::Display for Uncreatable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The game was unable to be created")
    }
}

impl std::error::Error for Uncreatable {}

/// This is the core of the Game ran by the [`Engine`](crate::Engine)
pub trait Game: Sized {
    /// Create the Game instance
    ///
    /// # Errors
    /// This is allowed to return an error. It will be printed in the console with either debug
    /// formatting in debug builds (`debug_assertions` enabled) and display in release builds
    ///
    /// If you use a closure as the game type this will always return an error
    fn create(engine: &mut crate::Engine) -> Result<Self, Box<dyn std::error::Error>>;

    #[allow(clippy::missing_errors_doc)]
    /// This is the core loop of the engine. It will be called every time the games needs a redraw.
    ///
    /// # Returns
    /// The return value of the function will determine if the game needs to be shut down or not.
    /// Returning `Ok(true)` will continue to the next frame
    /// `Ok(false)` will gracefully shut down the program
    /// `Err(_)` will print out the error onto stderr and stop the program
    fn update(&mut self, engine: &mut crate::Engine) -> Result<bool, Box<dyn std::error::Error>>;
    /// This function will be called when the input mode finishes.
    fn receive_input(&mut self, _engine: &mut crate::Engine, _input: String) {}
}

impl<F: (FnMut(&mut crate::Engine) -> Result<bool, Box<dyn std::error::Error>>) + 'static> Game
    for F
{
    fn create(_: &mut crate::Engine) -> Result<Self, Box<dyn std::error::Error>> {
        Err(Uncreatable.into())
    }
    fn update(&mut self, engine: &mut crate::Engine) -> Result<bool, Box<dyn std::error::Error>> {
        (self)(engine)
    }
}
