use std::time::Duration;

use sea_orm::{
    sqlx::types::chrono::Utc, ActiveModelTrait, ConnectOptions, Database, DbErr, EntityTrait, Set,
};
use tokio;

use model::user::{self, ActiveModel, Entity};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let mut conn_opt = ConnectOptions::new("sqlite://db.sqlite");
    conn_opt
        .max_connections(100)
        .min_connections(3)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10));

    let db = Database::connect(conn_opt).await?;
    println!("successfully connected to the database");

    // let new_user = user::ActiveModel {
    //     name: Set("wenshan".to_owned()),
    //     password: Set(Some("dfughaerjn".to_owned())),
    //     phone: Set(Some("18809389298".to_owned())),
    //     email: Set(Some("sdijsdnf@gmail.com".to_owned())),
    //     create_at: Set(Some(Utc::now().naive_utc())),
    //     ..Default::default()
    // };

    // let user = new_user.insert(&db).await?;
    // println!("insert user whit ID:{}", user.id);

    let users = Entity::find().all(&db).await?;
    for user in users {
        println!("user id {}, user name :{}", user.id, user.name);
    }

    // let update_user = Entity::find_by_id(2).one(&db).await?.unwrap();
    // let mut update_user: ActiveModel = update_user.into();
    // update_user.name = Set("laozhang".to_owned());
    // update_user.password = Set(Some("sdhfojner234".to_owned()));
    // let update_user = update_user.update(&db).await?;
    // println!("update user with ID:{}", update_user.id);

    // let del_user = Entity::find_by_id(5).one(&db).await?.unwrap();
    // let del_user: ActiveModel = del_user.into();
    // let del_user = del_user.delete(&db).await?;
    // println!("delete affected row:{}", del_user.rows_affected);

    Ok(())
}
