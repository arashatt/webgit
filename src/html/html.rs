use askama::Template;

pub struct PathPart {
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
 * Every page that extends this template needs these fields:
 *   repo: String          e.g. "linux.git"
 *   rev: String           ref or commit id used by the commit/commitdiff/tree tabs
 *   active: &'static str  summary | shortlog | log | commit | commitdiff | tree
 *                         | tag | tags | heads | blob | blame | search
 *   description: String   shown in the footer
 * Pages can add a second nav row by overriding the `nav_extra` block.
 */
pub struct RepoBase {
    pub repo: String,
    pub rev: String,
    pub active: &'static str,
    pub description: String,
}

/*
active = "tree"
  Fields:
    path_parts: Vec<PathPart>      PathPart { name, path }   empty at the repo root
    parent_path: Option<String>    None at the root, Some("") for a top-level dir, else the parent dir path
    entries: Vec<TreeEntry>
      TreeEntry { name, path, mode, size, is_dir }
      mode: "drwxr-xr-x" / "-rw-r--r--" / "lrwxrwxrwx"
      size: pre-formatted string, empty for directories
    */
pub struct TreeEntry {
    name: String,
    path: String,
    mode: String,
    is_dir: bool,
    size: String,
}
#[derive(Template)]
#[template(path = "tree.html")]
pub struct Tree {
    pub base: RepoBase,
    pub path_parts: Vec<PathPart>,
    pub parent_path: Option<String>,
    pub entries: Vec<TreeEntry>,
}
with_ctx!(Tree);
