use super::commands::CommandArgs;
use artemis_core::{
    core::artemis_collection,
    structs::{
        artifacts::os::{
            files::FileOptions,
            linux::{JournalOptions, LinuxSudoOptions, LogonOptions},
            macos::{
                EmondOptions, ExecPolicyOptions, FseventsOptions, LaunchdOptions,
                LoginitemsOptions, MacosGroupsOptions, MacosSudoOptions, MacosUsersOptions,
                SpotlightOptions, UnifiedLogsOptions,
            },
            processes::ProcessOptions,
        },
        toml::{ArtemisToml, Artifacts, Output},
    },
};
use clap::{arg, Subcommand};

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Acquire forensic artifacts
    Acquire {
        #[command(subcommand)]
        artifact: Option<CommandArgs>,
        /// Output format. JSON or JSONL.
        #[arg(long, default_value_t = String::from("JSON"))]
        format: String,
    },
}

/// Run the macOS collector and parse specified artifacts
pub(crate) fn run_collector(command: &Commands, output: Output) {
    let mut collector = ArtemisToml {
        system: String::from("macos"),
        output,
        artifacts: Vec::new(),
    };
    println!(
        "[artemis] Writing output to: {}",
        collector.output.directory
    );

    match command {
        Commands::Acquire { artifact, format } => {
            if artifact.is_none() {
                println!("No artifact provided");
                return;
            }

            let arti = artifact.as_ref().unwrap();
            collector.artifacts.push(setup_artifact(arti));

            if !format.is_empty() {
                collector.output.format = format.to_string().to_lowercase();
            }
        }
    }

    artemis_collection(&mut collector).unwrap();
}

/// Setup any artifact options. Only a few have options on macOS
fn setup_artifact(artifact: &CommandArgs) -> Artifacts {
    let mut collect = Artifacts {
        artifact_name: String::new(),
        filter: None,
        processes: None,
        files: None,
        unifiedlogs: None,
        script: None,
        emond: None,
        execpolicy: None,
        loginitems: None,
        launchd: None,
        fseventsd: None,
        users_macos: None,
        groups_macos: None,
        sudologs_macos: None,
        spotlight: None,
        journals: None,
        sudologs_linux: None,
        logons: None,
    };
    match artifact {
        CommandArgs::Processes {
            md5,
            sha1,
            sha256,
            metadata,
        } => {
            let options = ProcessOptions {
                md5: *md5,
                sha1: *sha1,
                sha256: *sha256,
                metadata: *metadata,
            };
            collect.processes = Some(options);
            collect.artifact_name = String::from("processes");
        }
        CommandArgs::Filelisting {
            md5,
            sha1,
            sha256,
            metadata,
            start_path,
            depth,
            regex_filter,
        } => {
            let options = FileOptions {
                md5: Some(*md5),
                start_path: start_path.to_string(),
                depth: Some(*depth),
                metadata: Some(*metadata),
                sha1: Some(*sha1),
                sha256: Some(*sha256),
                regex_filter: regex_filter.clone(),
            };
            collect.files = Some(options);
            collect.artifact_name = String::from("files");
        }
        CommandArgs::Chromiumhistory {} => collect.artifact_name = String::from("chromium-history"),
        CommandArgs::Chromiumdownloads {} => {
            collect.artifact_name = String::from("chromium-downloads")
        }
        CommandArgs::Firefoxdownloads {} => {
            collect.artifact_name = String::from("firefox-downloads")
        }
        CommandArgs::Firefoxhistory {} => collect.artifact_name = String::from("firefox-history"),
        CommandArgs::Cron {} => collect.artifact_name = String::from("cron"),
        CommandArgs::Shellhistory {} => collect.artifact_name = String::from("shell_history"),
        CommandArgs::Systeminfo {} => collect.artifact_name = String::from("systeminfo"),
        CommandArgs::Emond { alt_path } => {
            let options = EmondOptions {
                alt_path: alt_path.clone(),
            };
            collect.emond = Some(options);
            collect.artifact_name = String::from("emond");
        }
        CommandArgs::Fsevents { alt_file } => {
            let options = FseventsOptions {
                alt_file: alt_file.clone(),
            };
            collect.fseventsd = Some(options);
            collect.artifact_name = String::from("fseventsd");
        }
        CommandArgs::Execpolicy { alt_file } => {
            let options = ExecPolicyOptions {
                alt_file: alt_file.clone(),
            };
            collect.execpolicy = Some(options);
            collect.artifact_name = String::from("execpolicy");
        }
        CommandArgs::GroupsMacos { alt_path } => {
            let options = MacosGroupsOptions {
                alt_path: alt_path.clone(),
            };
            collect.groups_macos = Some(options);
            collect.artifact_name = String::from("groups");
        }
        CommandArgs::Launchd { alt_file } => {
            let options = LaunchdOptions {
                alt_file: alt_file.clone(),
            };
            collect.launchd = Some(options);
            collect.artifact_name = String::from("launchd");
        }
        CommandArgs::Loginitems { alt_file } => {
            let options = LoginitemsOptions {
                alt_file: alt_file.clone(),
            };
            collect.loginitems = Some(options);
            collect.artifact_name = String::from("loginitems");
        }
        CommandArgs::SafariDownloads {} => collect.artifact_name = String::from("safari-downloads"),
        CommandArgs::SafariHistory {} => collect.artifact_name = String::from("safari-history"),
        CommandArgs::UsersMacos { alt_path } => {
            let options = MacosUsersOptions {
                alt_path: alt_path.clone(),
            };
            collect.users_macos = Some(options);
            collect.artifact_name = String::from("users");
        }
        CommandArgs::SudologsMacos { logarchive_path } => {
            let options = MacosSudoOptions {
                logarchive_path: logarchive_path.clone(),
            };
            collect.sudologs_macos = Some(options);
            collect.artifact_name = String::from("sudologs-macos");
        }
        CommandArgs::Unifiedlogs {
            sources,
            logarchive_path,
        } => {
            let options = UnifiedLogsOptions {
                sources: sources.clone(),
                logarchive_path: logarchive_path.clone(),
            };
            collect.unifiedlogs = Some(options);
            collect.artifact_name = String::from("unifiedlogs");
        }
        CommandArgs::Spotlight {
            alt_path,
            include_additional,
        } => {
            let options = SpotlightOptions {
                alt_path: alt_path.clone(),
                include_additional: Some(*include_additional),
            };
            collect.spotlight = Some(options);
            collect.artifact_name = String::from("spotlight");
        }
        CommandArgs::Journals { alt_path } => {
            let options = JournalOptions {
                alt_path: alt_path.clone(),
            };
            collect.journals = Some(options);
            collect.artifact_name = String::from("journal");
        }
        CommandArgs::Logons { alt_file } => {
            let options = LogonOptions {
                alt_file: alt_file.clone(),
            };
            collect.logons = Some(options);
            collect.artifact_name = String::from("logon");
        }
        CommandArgs::SudologsLinux { alt_path } => {
            let options = LinuxSudoOptions {
                alt_path: alt_path.clone(),
            };
            collect.sudologs_linux = Some(options);
            collect.artifact_name = String::from("sudologs-linux");
        }
    }
    collect
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod tests {
    use super::{run_collector, setup_artifact, Commands};
    use crate::collector::macos::CommandArgs::{
        Chromiumdownloads, Chromiumhistory, Cron, Emond, Execpolicy, Filelisting, Firefoxdownloads,
        Firefoxhistory, Fsevents, GroupsMacos, Journals, Launchd, Loginitems, Logons, Processes,
        SafariDownloads, SafariHistory, Shellhistory, Spotlight, SudologsLinux, SudologsMacos,
        Systeminfo, Unifiedlogs, UsersMacos,
    };
    use artemis_core::structs::toml::Output;
    fn output() -> Output {
        let out = Output {
            name: String::from("local_collector"),
            endpoint_id: String::from("local"),
            collection_id: 0,
            directory: String::from("./tmp"),
            output: String::from("local"),
            format: String::from("json"),
            compress: false,
            filter_name: None,
            filter_script: None,
            url: None,
            api_key: None,
            logging: None,
        };

        out
    }

    #[test]
    fn test_run_collector_proc() {
        let command = Commands::Acquire {
            artifact: Some(Processes {
                md5: true,
                sha1: false,
                sha256: false,
                metadata: false,
            }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);
    }

    #[test]
    fn test_run_collector_files() {
        let command = Commands::Acquire {
            artifact: Some(Filelisting {
                md5: true,
                sha1: false,
                sha256: false,
                metadata: false,
                start_path: String::from("/"),
                depth: 1,
                regex_filter: None,
            }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);
    }

    #[test]
    fn test_run_macos_collector_root() {
        let command = Commands::Acquire {
            artifact: Some(Chromiumdownloads {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Chromiumhistory {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Firefoxdownloads {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Firefoxhistory {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Launchd { alt_file: None }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(UsersMacos { alt_path: None }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(SudologsMacos {
                logarchive_path: None,
            }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Cron {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Systeminfo {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(GroupsMacos { alt_path: None }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Execpolicy { alt_file: None }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Shellhistory {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Fsevents { alt_file: None }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Emond { alt_path: None }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(SafariDownloads {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(SafariHistory {}),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);
    }

    #[test]
    fn test_run_collector_logs() {
        let command = Commands::Acquire {
            artifact: Some(Unifiedlogs {
                sources: vec![String::from("Special")],
                logarchive_path: None,
            }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);
    }

    #[test]
    fn test_run_collector_spotlight() {
        let command = Commands::Acquire {
            artifact: Some(Spotlight {
                alt_path: None,
                include_additional: false,
            }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);
    }

    #[test]
    fn test_setup_artifact() {
        let result = setup_artifact(&Loginitems { alt_file: None });
        assert_eq!(result.artifact_name, "loginitems");
    }

    #[test]
    fn test_run_linux_collector_others() {
        let command = Commands::Acquire {
            artifact: Some(Logons { alt_file: None }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(Journals {
                alt_path: Some(String::from(".")),
            }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);

        let command = Commands::Acquire {
            artifact: Some(SudologsLinux { alt_path: None }),
            format: String::from("json"),
        };

        let out = output();
        run_collector(&command, out);
    }
}
