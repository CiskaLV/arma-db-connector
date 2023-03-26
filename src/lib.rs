
use std::vec;

use arma_rs::{arma, Extension, context, Group};

pub mod postgres;
pub mod handleconn;
pub mod config;

#[arma]
fn init() -> Extension {
    let exp = Extension::build()
        .version("0.1.0".to_owned())
        // .state(Config::new())
        // .group("sql", handleconn::group());
        .group("test", handleconn::group());
        // .group("g", generate_db_group());


    exp.finish()
}

fn generate_db_group() -> Group {
    let cfg = config::Config::new();

    let mut groups: Vec<Group> = vec![];

    for db in cfg.database {
        groups.push(
            Group::new()
                .state(handleconn::Database::new(&db.kind))
        )
    }

    Group::new().group("a", groups[0])
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn setup_sql() {
    //     let extension = init().testing();
    //     let (output, _) = unsafe { extension.call("sql:init", None) }; //Some(vec!["postgres".to_string()]))
    //     assert_eq!(output, "Connected to database: 8");
    // }
}