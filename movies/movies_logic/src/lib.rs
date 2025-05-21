use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: u16,
    pub director: String,
    pub actors: Vec<String>,
}

pub struct UpdateMovie {
    pub title: Option<String>,
    pub year: Option<u16>,
    pub director: Option<String>,
    pub actors: Option<Vec<String>>,
}

#[derive(Clone, Serialize)]
pub struct IdentifyableMovie {
    pub id: usize,
    
    #[serde(flatten)]
    pub movie: Movie,
}

impl IdentifyableMovie {
    pub fn new(id: usize, movie: Movie) -> Self {
        Self { id, movie }
    }
}

pub struct MovieStore {
    store: HashMap<usize, IdentifyableMovie>,
    next_id: AtomicUsize,
}

impl MovieStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn get_movies(&self) -> Vec<IdentifyableMovie> {
        self.store.values().cloned().collect()
    }

    pub fn get_movie(&self, id: usize) -> Option<IdentifyableMovie> {
        self.store.get(&id).cloned()
    }

    pub fn add_movie(&mut self, movie: Movie) -> IdentifyableMovie {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let movie = IdentifyableMovie::new(id, movie);
        self.store.insert(id, movie.clone());
        movie
    }

    pub fn update_movie(&mut self, id: usize, update: UpdateMovie) -> Option<IdentifyableMovie> {
        match self.store.get_mut(&id) {
            Some(movie) => {
                if let Some(title) = update.title {
                    movie.movie.title = title;
                }
                if let Some(year) = update.year {
                    movie.movie.year = year;
                }
                if let Some(director) = update.director {
                    movie.movie.director = director;
                }
                if let Some(actors) = update.actors {
                    movie.movie.actors = actors;
                }
                Some(movie.clone())
            }
            None => None,
        }
    }

    pub fn delete_movie(&mut self, id: usize) -> Option<IdentifyableMovie> {
        self.store.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie_store_operations() {
        // Create a new movie store
        let mut store = MovieStore::new();
        
        // Add a movie
        let movie = Movie {
            title: "The Matrix".to_string(),
            year: 1999,
            director: "Wachowski Sisters".to_string(),
            actors: vec!["Keanu Reeves".to_string(), "Laurence Fishburne".to_string()],
        };
        
        let added_movie = store.add_movie(movie);
        let id = added_movie.id;
        
        // Verify movie can be retrieved by ID
        let retrieved = store.get_movie(id).unwrap();
        assert_eq!(retrieved.movie.title, "The Matrix");
        assert_eq!(retrieved.movie.year, 1999);
        
        // Update the movie
        let update = UpdateMovie {
            title: Some("The Matrix Reloaded".to_string()),
            year: Some(2003),
            director: None,
            actors: Some(vec![
                "Keanu Reeves".to_string(), 
                "Laurence Fishburne".to_string(),
                "Carrie-Anne Moss".to_string(),
            ]),
        };
        
        let updated = store.update_movie(id, update).unwrap();
        assert_eq!(updated.movie.title, "The Matrix Reloaded");
        assert_eq!(updated.movie.year, 2003);
        assert_eq!(updated.movie.director, "Wachowski Sisters"); // Unchanged
        assert_eq!(updated.movie.actors.len(), 3);
        
        // Add another movie
        let second_movie = Movie {
            title: "Inception".to_string(),
            year: 2010,
            director: "Christopher Nolan".to_string(),
            actors: vec!["Leonardo DiCaprio".to_string(), "Ellen Page".to_string()],
        };
        
        store.add_movie(second_movie);
        
        // Get all movies and verify count
        let all_movies = store.get_movies();
        assert_eq!(all_movies.len(), 2);
        
        // Delete a movie and verify it's gone
        let deleted = store.delete_movie(id).unwrap();
        assert_eq!(deleted.movie.title, "The Matrix Reloaded");
        
        // Verify movie is gone
        assert!(store.get_movie(id).is_none());
        
        // Verify movie count is reduced
        let remaining = store.get_movies();
        assert_eq!(remaining.len(), 1);
    }
}
