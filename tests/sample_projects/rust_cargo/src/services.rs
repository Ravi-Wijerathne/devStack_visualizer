use crate::models::{CreateUserRequest, Post, User};
use std::collections::HashMap;
use std::error::Error;

pub struct UserService {
    users: HashMap<i32, User>,
    next_id: i32,
}

pub struct PostService {
    posts: HashMap<i32, Post>,
    next_id: i32,
}

impl UserService {
    pub fn new() -> Self {
        let mut service = Self {
            users: HashMap::new(),
            next_id: 1,
        };
        service.seed_data();
        service
    }

    fn seed_data(&mut self) {
        let users = vec![
            User::new(1, "alice", "alice@example.com"),
            User::new(2, "bob", "bob@example.com"),
            User::new(3, "charlie", "charlie@example.com"),
        ];
        for user in users {
            self.users.insert(user.id, user);
        }
        self.next_id = 4;
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        Ok(self.users.values().cloned().collect())
    }

    pub fn get_user_by_id(&self, id: i32) -> Option<User> {
        self.users.get(&id).cloned()
    }

    pub fn create_user(&mut self, req: CreateUserRequest) -> Result<User, Box<dyn Error>> {
        let user = User::new(self.next_id, &req.username, &req.email);
        self.users.insert(self.next_id, user.clone());
        self.next_id += 1;
        Ok(user)
    }
}

impl PostService {
    pub fn new() -> Self {
        Self {
            posts: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn get_all_posts(&self) -> Result<Vec<Post>, Box<dyn Error>> {
        Ok(self.posts.values().cloned().collect())
    }

    pub fn get_posts_by_user(&self, user_id: i32) -> Vec<Post> {
        self.posts
            .values()
            .filter(|p| p.user_id == user_id)
            .cloned()
            .collect()
    }

    pub fn create_post(&mut self, user_id: i32, title: &str, content: &str) -> Post {
        let post = Post::new(self.next_id, user_id, title, content);
        self.posts.insert(self.next_id, post.clone());
        self.next_id += 1;
        post
    }
}
