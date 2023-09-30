mod task;
pub use task::{
    task_action::TaskAction,
    task_settings::TaskSettings,
    task_trigger::{TaskIdleTrigger, TaskLogonTrigger},
    RunLevel, Task,
};

mod registered_task;
pub use registered_task::{RegisteredTask, TaskState};

pub use windows::core::{Error, Result};
pub use windows::Win32::Foundation::SYSTEMTIME;
