use std::fmt;

#[derive(Debug, Clone)]
pub enum ColorSet {
    WHITE,
    RED,
    GREEN,
    ORANGE,
    BLUE,
    YELLOW,
}

impl fmt::Display for ColorSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // On utilise le macro write! pour envoyer le texte dans le formatter 'f'
        let _ = write!(f, "{}", match self {
            Self::WHITE => "W",
            Self::RED => "R",
            Self::GREEN => "G",
            Self::ORANGE => "O",
            Self::BLUE => "B",
            Self::YELLOW => "Y"
        });
        Ok(())
    }
}
