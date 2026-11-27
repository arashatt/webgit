use git2::Blob;
use git2::Index;
use git2::Reference;
use git2::Repository as Repo;
use git2::Tree;
use std::path::Path;

pub struct Repository {
    name: String,
    pub repo: Repo,
    index: Index,
}

impl Repository {
    pub fn open(name: String) -> Result<Self, git2::Error> {
        let path_template = format!(
            "{}/{}",
            std::env::var("HOME").expect("Home variable doesn't exist"),
            name
        );
        let repo = Repo::open(&path_template)?;
        let index = repo.index().unwrap();
        Ok(Repository { name, repo, index })
    }
    pub fn repo(&self) -> Result<&Repo, git2::Error> {
        Ok(&self.repo)
    }
    pub fn head(&self) -> Result<Reference<'_>, git2::Error> {
        self.repo.head()
    }

    pub fn tree(&self) -> Result<Tree, git2::Error> {
        Ok(self.head()?.peel_to_tree().unwrap())
    }
    
    pub fn name(&self) -> String {
        self.name
    }
    pub fn index(&self) -> Result<Index, git2::Error> {
        Ok(self.repo.index().unwrap())
    }

    pub fn tree_iter(&self) {
        let tree = self.tree().unwrap();
    }
}
