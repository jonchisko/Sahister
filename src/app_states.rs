#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    SetMenu,
    CreditsMenu,
    InGame
}