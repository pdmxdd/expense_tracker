/* FORMS
Forms are Rocket's way of passing information from a POST request
to the server (here). 

Since Rust is statically typed, and compiled
we have to define a Form for each POST route.
*/

#[derive(FromForm)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(FromForm)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct CategoryForm {
    pub name: String,
    pub descrip: String,
}

#[derive(FromForm)]
pub struct ExpenseForm {
    pub category_id: String,
    pub name: String,
    pub amount: String,
}