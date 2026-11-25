# gitweb-style Askama templates

Put `templates/` next to your `Cargo.toml` and serve `static/` at `/static`.

## Routes the templates link to

| Route | Template |
|---|---|
| `/` (`?s=query`) | `project_list.html` |
| `/{repo}/summary` | `summary.html` |
| `/{repo}/shortlog`, `/{repo}/shortlog/{rev}` (`?page=N`) | `shortlog.html` |
| `/{repo}/history/{rev}/{*path}` (`?page=N`) | `shortlog.html` with a path-limited commit list |
| `/{repo}/log`, `/{repo}/log/{rev}` (`?page=N`) | `log.html` |
| `/{repo}/commit/{rev}` | `commit.html` |
| `/{repo}/commitdiff/{rev}` (`?parent=sha`) | `commitdiff.html` |
| `/{repo}/tree/{rev}`, `/{repo}/tree/{rev}/{*path}` | `tree.html` |
| `/{repo}/blob/{rev}/{*path}` | `blob.html` |
| `/{repo}/blame/{rev}/{*path}` | `blame.html` |
| `/{repo}/tags`, `/{repo}/heads` | `tags.html`, `heads.html` |
| `/{repo}/tag/{name}` | `tag.html` |
| `/{repo}/search?st=&s=&h=&page=` | `search.html` |
| `/{repo}/atom`, `/{repo}/rss` | `atom.xml`, `rss.xml` |
| `/{repo}/raw/{rev}/{*path}`, `/{repo}/patch/{rev}`, `/{repo}/commitdiff_plain/{rev}`, `/{repo}/snapshot/{rev}` | no template (plain responses) |

Axum 0.8 route syntax is shown (`{*path}`); use `/*path` on 0.7. Refs and paths are
percent-encoded in the templates with `urlencode_strict` (refs, so `feature/x` stays one
segment) and `urlencode` (paths).

## Template inheritance

```
base.html
├── project_list.html
├── error.html
└── repo_base.html
    ├── summary, shortlog, log, commit, commitdiff
    ├── tree, blob, blame
    └── tags, heads, tag, search
```

Every page extending `repo_base.html` needs `repo`, `rev`, `active`, `description`.
No block is nested inside another block, so there is nothing version-sensitive there.
A page can add a second nav row by overriding `nav_extra`.

## Shared item types (field lists)

- RefLabel: `kind` (`head` | `tag` | `ref`), `name`
- CommitRow: `id`, `age`, `date`, `author`, `subject`, `refs: Vec<RefLabel>`
- LogEntry: `id`, `age`, `date`, `author`, `subject`, `body`, `refs`
- TagRow: `name`, `age`, `subject`, `annotated: bool`, `commit_id`
- HeadRow: `name`, `age`, `subject`, `current: bool`
- ProjectRow: `name`, `description`, `owner`, `age`, `age_class` (`age0` | `age1` | `age2` | `noage`)
- PathPart: `name`, `path`
- ChangedFile, CommitDetail, DiffFile, Hunk, DiffLine, TreeEntry, BlameLine, TagDetail, FeedEntry:
  see the comment at the top of the template that uses them.

Each page's own fields are listed in the `{# ... #}` comment at the top of its template.
