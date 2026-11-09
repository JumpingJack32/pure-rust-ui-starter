// src/lib.rs
pub mod components;

// Re-export for cleaner imports:
//   use ui_components::Button;
//   use ui_components::ThemeProvider;
//   use ui_components::ModeToggle;

pub use components::ui::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
pub use components::ui::avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage,
};
pub use components::ui::badge::Badge;
// pub use components::ui::button::{Button, ButtonVariant};
pub use components::ui::button::{button_variants, Button, ButtonSize, ButtonVariant};
pub use components::ui::card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
pub use components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
pub use components::ui::input::Input;
pub use components::ui::navigation_menu::{
    NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
};
pub use components::ui::sheet::{
    Sheet, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetTitle,
};

pub use components::mode_toggle::ModeToggle;
pub use components::theme_provider::Theme;
pub use components::theme_provider::ThemeContext;
pub use components::theme_provider::ThemeProvider;
