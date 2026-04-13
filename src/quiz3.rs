//Design a Repository trait with associated Error, Id, and Item types. Implement it for an in-memory store and demonstrate compile-time type safety.

use std::{collections::HashMap, convert::Infallible, fmt::Debug};

trait Repository {
    type Error;
    type Id;
    type Item;

    fn get(&self, id: &Self::Id) -> Result<Option<&Self::Item>, Self::Error>;
    fn insert(&mut self, item: Self::Item) -> Result<Self::Id, Self::Error>;
    fn delete(&mut self, id: Self::Id) -> Result<bool, Self::Error>;
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    email: String,
}

struct InMemoryUserRepo {
    data: HashMap<u64, User>,
    next_id: u64,
}

impl InMemoryUserRepo {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Repository for InMemoryUserRepo {
    type Error = Infallible;
    type Id = u64;
    type Item = User;

    fn get(&self, id: &Self::Id) -> Result<Option<&User>, Self::Error> {
        Ok(self.data.get(id))
    }

    fn insert(&mut self, item: Self::Item) -> Result<Self::Id, Self::Error> {
        let following_id = self.next_id;
        self.data.entry(following_id).or_insert(item);
        self.next_id += 1;
        Ok(following_id)
    }

    fn delete(&mut self, id: Self::Id) -> Result<bool, Self::Error> {
        Ok(self.data.remove(&id).is_some())
    }
}

fn create_and_fetch<R: Repository>(repo: &mut R, item: R::Item) -> Result<(), R::Error>
where
    R::Item: Debug,
    R::Id: Debug,
{
    let id = repo.insert(item)?;
    println!("Created and inserted user with Id {:?}", id);

    let user = repo.get(&id)?;
    println!(" user: {:?}", user);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_create_and_fetch() {
        let mut repo = InMemoryUserRepo::new();
        create_and_fetch(
            &mut repo,
            User {
                name: "Alice".into(),
                email: "alice@example.com".into(),
            },
        )
        .unwrap();

        create_and_fetch(
            &mut repo,
            User {
                name: "femi".into(),
                email: "femi@example.com".into(),
            },
        )
        .unwrap();

        assert_eq!(repo.data.len(), 2);
        assert_eq!(repo.data.get(&1).unwrap().name, "Alice");
        assert_eq!(repo.data.get(&2).unwrap().name, "Femi".to_lowercase());
    }
}
