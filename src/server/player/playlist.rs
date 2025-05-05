use rand::{self, Rng};
#[derive(Debug, Clone)]
pub(crate) struct Playlist {
    current: usize,
    files: Vec<String>,
    random: bool,
}

impl Playlist {
    /// Creates a new playlist with the given files.
    ///
    /// # Arguments
    ///
    /// * `files` - A vector of file paths to be included in the playlist.
    /// * `random` - A boolean indicating whether the playlist should be played in random order.
    ///
    /// # Returns
    ///
    /// A new instance of `Playlist`.
    pub(crate) fn new(files: &Vec<String>, random: bool) -> Self {
        Playlist {
            current: files.len() - 1,
            files: files.clone(),
            random,
        }
    }
}

impl Iterator for Playlist {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.random {
            let mut rng = rand::rng();
            self.current = rng.random_range(0..self.files.len());
        } else {
            self.current = (self.current + 1) % self.files.len();
        };

        self.files.get(self.current).map(|s| s.to_string())
    }
}
