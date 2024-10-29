{
  "query": "SELECT posts.id, posts.title, posts.body, posts.published FROM posts WHERE (posts.published = $1) LIMIT $2;  -- binds: [true, 5]",
  "duration": "5131 µs"
}
