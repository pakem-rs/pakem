#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use pakem::prelude::*;

//                            USER MODEL
// Di Laravel, ini adalah class Eloquent (`User extends Model`).
// Cukup tambahkan `#[derive(PakemModel)]` untuk mendapatkan fungsi CRUD otomatis:
// `User::find(id)`, `User::all()`, dll.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, PakemModel)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
