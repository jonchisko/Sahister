#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    OptionsMenu,
    CreditsMenu,
    InGame
}