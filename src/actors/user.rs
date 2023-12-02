use crate::models::user_model::{CreateUser, User};
use actix_web::Result;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

impl User {
    // Add a new user to the database
    //
    // # Parameters
    //
    // * `conn` - The database connection
    // * `data` - The user data to create, including their full name, email address, and password
    //
    // # Returns
    //
    // A `UserDetails` struct containing the details of the newly created user, including their full name, email address, password, and creation and update timestamps
    pub fn add_user(conn: &mut PgConnection, data: &CreateUser) -> Result<User, DbError> {
        use crate::schema::users::dsl::*;
        let current_time = Utc::now().naive_utc();
        let new_user = User {
            id: uuid::Uuid::new_v4(),
            full_name: data.full_name.to_owned(),
            email: data.email.to_owned(),
            password: hash(data.password.as_bytes(), DEFAULT_COST)?,
            created_at: current_time,
            updated_at: current_time,
        };

        // Insert the new user into the database
        diesel::insert_into(users).values(&new_user).execute(conn)?;

        // Return the new user
        Ok(new_user)
    }

    // Find a user by their email address in the database
    //
    // # Parameters
    //
    // * `conn` - The database connection
    // * `user_email` - The email address of the user to find
    //
    // # Returns
    //
    // A `User` struct if a user with the specified email address was found, or an error if not
    pub fn find_user_by_email(conn: &mut PgConnection, user_email: &str) -> Result<User, DbError> {
        use crate::schema::users::dsl::*;

        // Attempt to find the user by email
        let result = users.filter(email.eq(user_email)).first::<User>(conn)?;

        Ok(result)
    }

    /// Deletes a user from the database based on their email address
    ///
    /// # Parameters
    ///
    /// * `conn` - The database connection
    /// * `user_email` - The email address of the user to delete
    ///
    /// # Returns
    ///
    /// A `String` containing the email address of the deleted user
    pub fn delete_user(conn: &mut PgConnection, user_email: &str) -> Result<String, DbError> {
        use crate::schema::users::dsl::*;

        // Attempt to find the user by email
        let result = users.filter(email.eq(user_email)).first::<User>(conn)?;

        // Delete the user from the database
        diesel::delete(users.filter(email.eq(user_email))).execute(conn)?;

        // Return the email address of the deleted user
        Ok(result.email)
    }
}
