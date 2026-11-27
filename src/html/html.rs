use askama::Template;
use crate::git:repository::Repository;


struct PathPart {
    name: String,
    path: String,
}

macro_rules! with_ctx {
    ($($page:ident),+ $(,)?) => {$(
        impl std::ops::Deref for $page {
            type Target = RepoBase;
            fn deref(&self) -> &RepoBase{
                &self.base
            }
        }
    )+};
}

/*
 * Every page extending this template needs these fields (put them in one shared
 * RepoCtx struct and Deref to it, see src/ctx.rs). Page structs must NOT reuse
 * these names for their own fields.
 *   repo: String                   e.g. "linux.git"
 *   head: String                   current ref (branch/tag name), never empty
 *   branches: Vec<BranchOpt>       BranchOpt { name, selected: bool }  (branch switcher)
 *   active: &'static str           summary | refs | log | tree | commit | diff | stats | about
 *   description: String
 *   owner: String
 *   has_stats: bool                show the stats tab
 *   has_about: bool                show the about tab
 *   search_type: String            grep | author | committer | range
 *   search_text: String
*/
pub struct RepoBase {
    pub repo: String,
    pub head: String,
    pub branches: Vec<BranchOpt>,
    pub active: &'static str,
    pub description: String,
    owner: String,
    has_stats: bool,
    has_about: bool,
    search_type: String,
    search_text: String,
}

impl RepoBase {
    pub fn new(repo: Repository) -> Self{
        let tree = 
        Self{
            repo: repo.name(),  
        }
    }   
}


struct TreeEntry {
    name: String,
    path: String,
    mode: String,
    size: i32, 
    is_dir: bool ,
    is_submodul: bool,
}
impl From<TreeEntry> for git2::TreeEntry {
    fn from(tree_entry: git2::TreeEntry) -> Self{
     let num2str = |a: i32| match a {
        100644 => "-rw-r--r--".to_owned(),
        100755 => "Executable file".to_owned(),
        120000 => "lrwxrwxrwx".to_owned(),
        _ => "Not Defined".to_owned(),
    };

     Self {
         name: format!("This is just a debugging hack and you shouldn't see this in the production, {}, {} {}",
                   tree_entry.name(),
                   tree_entry.filemode(),
                   tree_entry.filemode_raw()
                   ),
         path: "TODO placeholder".to_owned(),
         mode: num2str(tree_entry.filemode()),
         size: 0,// TODO!
         is_dir: tree_entry.kind() == git2::ObjectType::Tree? true, false,
         is_submodule: tree_entry.kind() == git2::ObjectType::Commit? true, false,
     }

    } 
}
 /*
  active = "tree"
  Fields:
    path: String                   current directory, "" at the root
    path_parts: Vec<PathPart>      PathPart { name, path }, empty at the root
    parent_path: Option<String>    None at the root, Some("") for a top-level dir, else the parent dir
    entries: Vec<TreeEntry>
      TreeEntry { name, path, mode, size, is_dir, is_submodule }
      mode: "d---------" / "-rw-r--r--" / "lrwxrwxrwx"; size: pre-formatted, "" for directories
    show_blame: bool               show the blame button on files
*/

#[derive(Template)]
#[template(path = "tree.html")]
pub struct Tree {
    pub base: RepoBase,
    pub path_parts: Vec<PathPart>,
    pub parent_path: Option<String>,
    pub entries: Vec<TreeEntry>,
    pub blame_bool: bool,
}
with_ctx!(Tree);
