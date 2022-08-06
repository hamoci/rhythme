#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    StartMenu,
    SelectMenu,
    InGame,
    Result,
}