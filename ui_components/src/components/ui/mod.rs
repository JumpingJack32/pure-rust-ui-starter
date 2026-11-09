// src/components/ui/mod.rs

pub mod accordion;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod dropdown_menu;
pub mod input;
pub mod navigation_menu;
pub mod sheet;

// Re-export all Avatar subcomponents cleanly
pub use accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
pub use avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};
pub use badge::Badge;
pub use button::{Button, ButtonVariant};
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
pub use input::Input;
pub use navigation_menu::{
    NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
};
pub use sheet::{Sheet, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetTitle};
