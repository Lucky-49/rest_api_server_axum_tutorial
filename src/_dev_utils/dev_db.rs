use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

type Db = Pool<Postgres>;

//Хардкод для предотвращения использования в продакшене
// "postgres" - юзернэйм подключение к бд
// "welcome" -пароль для подключения к бд
// "postgres_axum" - название бд
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost:5433/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost:5433/app_db";


//sql файлы
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

    //создание базы данных  app_db/app_user юзером postgres
    {
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    //Получение данных из файлов в папке dev_initial
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    paths.sort(); //при использовании сортировки необходимо чтобы файлы имели последовательность наименований (00, 01, 02, 03 и т.д)


    //Выполняем каждый sql файл
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/"); //замена слэша для винды

            //используем только *.sql и пропускаем повторное использование SQL_RECREATE_DB
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?;
            }
        }
    }

    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

    //Читаем содержимое файла
    let content = fs::read_to_string(file)?;

    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;

    }

    Ok(())
}

//создаем пул баз данных
async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}

