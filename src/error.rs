use std::{fmt, io};
// use tokio_cron_scheduler::JobSchedulerError;

#[derive(Debug)]
pub enum Error {
    BackupHostConfigError(String, String),
    BucketConfigError(String, String),
    BackblazeLoginError(String),
    MariaDbDumpError(String, String),
    DatabaseCompressionError(String, String),
    SchedulerError(String),
    IoError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Error::BackupHostConfigError(backup_host, db_name) => format!(
                 "An error occured while parsing the host config of the {db_name} database, the specified backup host, {backup_host} is not configured in the config file - This backup WILL be skipped."
            ),
            Error::BucketConfigError(bucket, db_name) => format!(
                "An error occured while parsing the bucket config of the {db_name} database, the specified bucket host, {bucket} is not configured in the config file - This backup WILL be skipped."
            ),
            Error::BackblazeLoginError(err) => format!("An error occured while logging into backblaze, did you set the right details?\nError: {err}"),
            Error::MariaDbDumpError(err, db_name) => format!("It seems mariadb-dump exited with an error for the backup of database {db_name}, this backup will not complete\nError: {err}"),
            Error::IoError(err) => format!("An error occured while modifying a file/folder: {err}"),
            Error::DatabaseCompressionError(err, db_name) => format!("An error occured while compressing the backup of database {db_name}, this backup will not complete\nError: {err}"),
            Error::SchedulerError(err) => format!("An error occured while scheduling a backup job: {err}"),
        };
        write!(f, "{}", str)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

// impl From<JobSchedulerError> for Error {
//     fn from(error: JobSchedulerError) -> Self {
//         Error::SchedulerError(error.to_string())
//     }
// }
