use crate::apps::common::models::{ListData, PageParams};
use crate::apps::system::models::sys_job::StatusReq;
use crate::database::{db_conn, DB};
use crate::tasks;
use chrono::{Local, NaiveDateTime};
use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use super::super::entities::{prelude::SysJob, sys_job};
use super::super::models::sys_job::{AddReq, DeleteReq, EditReq, SearchReq};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    search_req: SearchReq,
) -> Result<ListData<sys_job::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysJob::find();

    if let Some(x) = search_req.job_name {
        s = s.filter(sys_job::Column::JobName.contains(&x));
    }

    if let Some(x) = search_req.job_group {
        s = s.filter(sys_job::Column::JobGroup.contains(&x));
    }
    if let Some(x) = search_req.status {
        s = s.filter(sys_job::Column::Status.eq(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await.map_err(BadRequest)?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_job::Column::JobId)
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await.map_err(BadRequest)?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(BadRequest)?;

    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

pub async fn check_job_add_is_exist<'a, C>(db: &'a C, job_name: &str, task_id: i64) -> Result<bool>
where
    C: ConnectionTrait<'a>,
{
    let c1 = SysJob::find()
        .filter(sys_job::Column::JobName.eq(job_name))
        .count(db)
        .await
        .map_err(BadRequest)?;
    let c2 = SysJob::find()
        .filter(sys_job::Column::TaskId.eq(task_id))
        .count(db)
        .await
        .map_err(BadRequest)?;
    Ok(c1 > 0 || c2 > 0)
}

pub async fn check_job_edit_is_exist<'a, C>(
    db: &'a C,
    job_name: &str,
    task_id: i64,
    job_id: &str,
) -> Result<bool>
where
    C: ConnectionTrait<'a>,
{
    let c1 = SysJob::find()
        .filter(sys_job::Column::JobName.eq(job_name))
        .filter(sys_job::Column::JobId.ne(job_id))
        .count(db)
        .await
        .map_err(BadRequest)?;
    let c2 = SysJob::find()
        .filter(sys_job::Column::TaskId.eq(task_id))
        .filter(sys_job::Column::JobId.ne(job_id))
        .count(db)
        .await
        .map_err(BadRequest)?;
    Ok(c1 > 0 || c2 > 0)
}

/// add 添加
pub async fn add<'a, C>(db: &'a C, req: AddReq, user_id: String) -> Result<String>
where
    C: ConnectionTrait<'a>,
{
    //  检查字典类型是否存在
    if check_job_add_is_exist(db, &req.job_name, req.task_id).await? {
        return Err(Error::from_string("任务已存在", StatusCode::BAD_REQUEST));
    }
    let uid = scru128::scru128_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let next_time = tasks::get_next_task_run_time(req.cron_expression.to_string());
    let status = req.status.unwrap_or_else(|| "1".to_string());
    let add_data = sys_job::ActiveModel {
        job_id: Set(uid.clone()),
        task_id: Set(req.task_id),
        task_count: Set(req.task_count),
        job_name: Set(req.job_name),
        job_params: if let Some(x) = req.job_params {
            Set(Some(x))
        } else {
            NotSet
        },
        job_group: Set(req.job_group),
        invoke_target: Set(req.invoke_target),
        cron_expression: Set(req.cron_expression),
        misfire_policy: Set(req.misfire_policy),
        concurrent: if let Some(x) = req.concurrent {
            Set(Some(x))
        } else {
            NotSet
        },
        status: Set(status.clone()),
        remark: Set(Some(req.remark.unwrap_or_else(|| "".to_string()))),
        next_time: Set(next_time),
        create_by: Set(user_id),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    SysJob::insert(add_data.clone())
        .exec(db)
        .await
        .map_err(BadRequest)?;
    if status.as_str() == "1" {
        tasks::run_circles_task(uid.clone())
            .await
            .expect("任务添加失败");
    };

    let res = format!("{}添加成功", uid);

    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = SysJob::delete_many();

    s = s.filter(sys_job::Column::JobId.is_in(delete_req.job_ids));

    //开始删除
    let d = s
        .exec(db)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::BAD_REQUEST))?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(Error::from_string(
            "你要删除的字典类型不存在",
            StatusCode::BAD_REQUEST,
        )),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: EditReq, user_id: String) -> Result<String> {
    //  检查字典类型是否存在
    if check_job_edit_is_exist(db, &req.job_name, req.task_id, &req.job_id).await? {
        return Err(Error::from_string("任务已存在", StatusCode::BAD_REQUEST));
    }
    let uid = req.job_id;
    let s_s = get_by_id(db, uid.clone()).await?;
    let s_r: sys_job::ActiveModel = s_s.clone().into();
    let next_time = tasks::get_next_task_run_time(req.cron_expression.to_string());
    let status = req.status.unwrap_or_else(|| "1".to_string());
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_job::ActiveModel {
        job_id: Set(uid.clone()),
        task_id: Set(req.task_id),
        task_count: Set(req.task_count),
        job_name: Set(req.job_name),
        job_params: if let Some(x) = req.job_params {
            Set(Some(x))
        } else {
            NotSet
        },
        job_group: Set(req.job_group),
        invoke_target: Set(req.invoke_target),
        cron_expression: Set(req.cron_expression),
        misfire_policy: Set(req.misfire_policy),
        concurrent: if let Some(x) = req.concurrent {
            Set(Some(x))
        } else {
            NotSet
        },
        next_time: Set(next_time),
        status: Set(status.clone()),
        remark: Set(Some(req.remark.unwrap_or_else(|| "".to_string()))),
        update_by: Set(Some(user_id)),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    act.update(db).await.map_err(BadRequest)?;
    match status.clone().as_str() {
        "1" => {
            tasks::run_circles_task(uid.clone())
                .await
                .expect("任务执行失败");
        }
        "0" => {
            tasks::delete_job(s_s.clone().task_id.try_into().unwrap())
                .await
                .expect("任务删除失败");
        }
        _ => return Err(Error::from_string("状态值错误", StatusCode::BAD_REQUEST)),
    };
    Ok(format!("{}修改成功", uid))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
pub async fn get_by_id<'a, C>(db: &'a C, job_id: String) -> Result<sys_job::Model>
where
    C: ConnectionTrait<'a>,
{
    let s = SysJob::find()
        .filter(sys_job::Column::JobId.eq(job_id))
        .one(db)
        .await
        .map_err(BadRequest)?;

    let res = match s {
        Some(m) => m,
        None => return Err(Error::from_string("没有找到数据", StatusCode::BAD_REQUEST)),
    };
    Ok(res)
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_active_job(db: &DatabaseConnection) -> Result<Vec<sys_job::Model>> {
    let s = SysJob::find()
        // .filter(sys_job::Column::DeletedAt.is_null())
        .filter(sys_job::Column::Status.eq("1".to_string()))
        .order_by(sys_job::Column::JobId, Order::Asc)
        .all(db)
        .await
        .map_err(BadRequest)?;
    Ok(s)
}

/// delete 完全删除
pub async fn set_status(db: &DatabaseConnection, req: StatusReq) -> Result<String> {
    let job = get_by_id(db, req.job_id.clone()).await?;
    sys_job::Entity::update_many()
        .col_expr(sys_job::Column::Status, Expr::value(req.status.clone()))
        .filter(sys_job::Column::JobId.eq(req.job_id.clone()))
        .exec(db)
        .await
        .map_err(BadRequest)?;
    match req.status.clone().as_str() {
        "1" => {
            tasks::run_circles_task(job.clone().job_id)
                .await
                .expect("任务执行失败");
        }
        "0" => {
            tasks::delete_job(job.clone().task_id.try_into().unwrap())
                .await
                .expect("任务删除失败");
        }
        _ => return Err(Error::from_string("状态值错误", StatusCode::BAD_REQUEST)),
    };
    Ok(format!("{}修改成功", req.job_id))
}