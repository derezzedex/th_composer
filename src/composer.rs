use std::collections::HashSet;
use anyhow::{anyhow, Result};

use crate::technique::Technique;
use crate::challenge::Challenge;

pub struct ComposerSettings{
    available: HashSet<Technique>,
    // TODO: Add flag settings (size, possible chars, etc.)
    // TODO: Add paths to resources (images, text, html)
}

// Builder pattern
impl ComposerSettings{
    pub fn new() -> ComposerSettings{
        ComposerSettings{
            available: HashSet::new(),
        }
    }

    pub fn with(mut self, technique: Technique) -> ComposerSettings{
        self.available.insert(technique);

        self
    }
}

pub struct Composer{
    settings: ComposerSettings,

    players: usize,
    challenges: Vec<Challenge>,
}

impl Composer{
    pub fn new(settings: ComposerSettings, players: usize) -> Composer{
        let challenges = Vec::new();

        Composer{
            settings,

            players,
            challenges,
        }
    }

    pub fn add(&mut self, challenge: Challenge) -> Result<usize>{
        for technique in &challenge.steps{
            if !self.settings.available.contains(technique){
                return Err(anyhow!("{:?} is not available on this composer!", technique))
            }
        }

        self.challenges.push(challenge);
        Ok(self.challenges.len() - 1)
    }
}
