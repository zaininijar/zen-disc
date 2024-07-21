use crate::db;
use crate::error_handler::CustomError;
use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Posts {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Posts {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let posts = posts::table.load::<Posts>(&mut conn)?;
        Ok(posts)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let post = posts::table.filter(posts::id.eq(id)).first(&mut conn)?;
        Ok(post)
    }

    pub fn create(post: Post) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let post = Post::from(post);
        let post = diesel::insert_into(posts::table)
            .values(post)
            .get_result(&mut conn)?;
        Ok(post)
    }

    pub fn update(id: i32, post: Post) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let post = diesel::update(posts::table)
            .filter(posts::id.eq(id))
            .set(post)
            .get_result(&mut conn)?;
        Ok(post)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(posts::table.filter(posts::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl Post {
    fn from(post: Post) -> Post {
        Post {
            title: post.title,
            body: post.body,
            published: post.published,
        }
    }
}
