use catalytic_table_to_struct::transformer::DefaultTransformer;
use std::env::current_dir;

fn main() {
    dotenvy::dotenv().expect("Problem during dotenv environment variable setup!");

    catalytic_table_to_struct::generate(
        &current_dir().unwrap().join("src").join("generated"),
        DefaultTransformer,
    );
}
