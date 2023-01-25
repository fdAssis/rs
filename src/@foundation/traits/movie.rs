pub struct Movie {
    title: String,
    director: String,
    release_year: u32,
    genre: String,
}

pub trait Details {
    fn description(&self) -> String;
    fn years_since_release(&self) -> u32;
}

impl Movie {
    pub fn new(title: String, director: String, release_year: u32, genre: String) -> Self {
        Movie {
            title,
            director,
            release_year,
            genre,
        }
    }
}

impl Details for Movie {
    fn description(&self) -> String {
        format!(
            "{}, released in {}, is a {} movie directed by {}.",
            self.title, self.release_year, self.genre, self.director
        )
    }

    fn years_since_release(&self) -> u32 {
        2022u32 - self.release_year
    }
}
