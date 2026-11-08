// src/components/mod.rs
pub mod mode_toggle;
pub mod theme_provider;
pub mod ui;

// Optional: Re-export for flatter imports
pub use mode_toggle::ModeToggle;
pub use theme_provider::ThemeProvider;
