mod clean;
mod config;

use std::{
    collections::HashSet,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEvent, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, TableState, Tabs, Wrap,
    },
    Frame,
};
use sysinfo::{ProcessStatus, ProcessesToUpdate, System};

const TABS: [&str; 7] = ["Processes", "Services", "Disk", "Cleaner", "Automation", "Shell", "Network"];
const TAB_COLORS: [Color; 7] = [Color::Cyan, Color::Green, Color::Yellow, Color::Magenta, Color::Blue, Color::Red, Color::White];
/// Shown instead of "saved" when the on-disk config.toml cannot be parsed and
/// Config::save() therefore refuses to overwrite it.
const SAVE_BLOCKED: &str = "config.toml unparseable — change NOT saved (fix or delete it)";

struct Service {
    unit: String,
    load: String,
    active: String,
    sub: String,
    description: String,
}

struct Proc {
    pid: u32,
    name: String,
    state: ProcessStatus,
    cpu: f32,
    mem: u64,
}

enum Mode {
    Normal,
    Command,
    Sudo,
}

struct State {
    tab: usize,
    mode: Mode,
    cmd: String,
    sudo_pass: String,
    status: String,
    sys: System,
    procs: Vec<Proc>,
    proc_state: TableState,
    proc_filter: String,
    proc_marks: HashSet<u32>,
    services: Vec<Service>,
    svc_state: TableState,
    disks: Vec<(String, u64, u64)>,
    bigdirs: Vec<(String, u64)>,
    clean_targets: Vec<(String, u64, bool, bool)>,
    clean_state: TableState,
    autom: config::Automation,
    autom_items: Vec<String>,
    autom_state: TableState,
    confirm_clean: bool,
    last_watch_check: Instant,
    alerts: Vec<String>,
    shell_log: Vec<String>,
    tabs_rect: Rect,
    net_fns: Vec<(String, String, bool)>,
    net_state: TableState,
    net_live: bool,
    net_text: Option<Vec<String>>,
    net_nets: sysinfo::Networks,
    net_hist: std::collections::VecDeque<(u64, u64)>,
    net_last: Instant,
}

fn proc_state_label(st: &ProcessStatus) -> (&'static str, Color) {
    match st {
        ProcessStatus::Run => ("RUNNING", Color::Green),
        ProcessStatus::Sleep => ("SLEEPING", Color::Gray),
        ProcessStatus::Stop => ("STOPPED", Color::Yellow),
        ProcessStatus::Zombie => ("ZOMBIE", Color::Red),
        ProcessStatus::Tracing => ("TRACING", Color::Magenta),
        ProcessStatus::Dead => ("DEAD", Color::DarkGray),
        ProcessStatus::Idle => ("IDLE", Color::DarkGray),
        ProcessStatus::Wakekill => ("WAKEKILL", Color::Blue),
        ProcessStatus::Parked => ("PARKED", Color::DarkGray),
        ProcessStatus::LockBlocked => ("LOCKED", Color::Red),
        ProcessStatus::Waking => ("WAKING", Color::Blue),
        ProcessStatus::UninterruptibleDiskSleep => ("DISK WAIT", Color::Magenta),
        ProcessStatus::Suspended => ("SUSPENDED", Color::Yellow),
        ProcessStatus::Unknown(_) => ("UNKNOWN", Color::DarkGray),
    }
}

/// Ground-truth process state straight from /proc (sysinfo's mapping drifts).
fn proc_status(pid: u32) -> ProcessStatus {
    std::fs::read_to_string(format!("/proc/{pid}/stat"))
        .ok()
        .and_then(|c| c.rsplit(')').next().map(|s| s.trim().chars().next().unwrap_or('?')))
        .map(|c| match c {
            'R' => ProcessStatus::Run,
            'S' => ProcessStatus::Sleep,
            'D' => ProcessStatus::UninterruptibleDiskSleep,
            'Z' => ProcessStatus::Zombie,
            'T' => ProcessStatus::Stop,
            't' => ProcessStatus::Tracing,
            'X' => ProcessStatus::Dead,
            'x' => ProcessStatus::Dead,
            'I' => ProcessStatus::Idle,
            _ => ProcessStatus::Unknown(0),
        })
        .unwrap_or(ProcessStatus::Unknown(0))
}

impl State {
    fn new() -> Self {
        let (cfg, cfg_warn) = config::Config::load();
        let mut s = Self {
            tab: 0,
            mode: Mode::Normal,
            cmd: String::new(),
            sudo_pass: String::new(),
            status: String::from("vam ready — `:` command · `:help` for syntax · Tab switches tabs"),
            sys: System::new_all(),
            procs: vec![],
            proc_state: TableState::default(),
            proc_filter: String::new(),
            proc_marks: HashSet::new(),
            services: vec![],
            svc_state: TableState::default(),
            disks: vec![],
            bigdirs: vec![],
            clean_targets: vec![],
            clean_state: TableState::default(),
            autom: cfg.automation,
            autom_items: vec![],
            autom_state: TableState::default(),
            confirm_clean: false,
            last_watch_check: Instant::now(),
            alerts: vec![],
            shell_log: vec!["vam shell ready — type any command".into()],
            tabs_rect: Rect::default(),
            net_fns: vec![
                ("Live Speed Monitor".into(), "Real-time download/upload speeds + metrics sub-TUI".into(), false),
                ("Wi-Fi Networks".into(), "List visible Wi-Fi networks (nmcli)".into(), false),
                ("Active Connections".into(), "Active TCP/UDP connections (ss -tup)".into(), false),
                ("Ping Gateway".into(), "Ping the default gateway 4 times".into(), false),
                ("Flush DNS Cache".into(), "Flush resolver cache (resolvectl / nmcli)".into(), false),
            ],
            net_state: TableState::default(),
            net_live: false,
            net_text: None,
            net_nets: sysinfo::Networks::new_with_refreshed_list(),
            net_hist: std::collections::VecDeque::new(),
            net_last: Instant::now(),
        };
        s.rebuild_autom_items();
        s.refresh_procs();
        s.refresh_services();
        s.refresh_disk();
        s.refresh_clean();
        s.net_state.select(Some(0));
        s.autom_state.select(Some(0));
        if let Some(e) = cfg_warn {
            s.status = format!("config invalid — running defaults, changes will NOT be saved ({e})");
        }
        s
    }

    /// Persists `autom` to disk. Returns false when the write was refused.
    fn persist_automation(&self) -> bool {
        config::Config { automation: self.autom.clone() }.save()
    }

    fn rebuild_autom_items(&mut self) {
        self.autom_items = vec![
            format!("prune_yay_cache        {}", onoff(self.autom.prune_yay_cache)),
            format!("prune_thumbnails       {}", onoff(self.autom.prune_thumbnails)),
            format!("prune_mesa_shader      {}", onoff(self.autom.prune_mesa_shader)),
            format!("prune_flatpak_caches   {}", onoff(self.autom.prune_flatpak_caches)),
            format!("low_disk_watch (>={}%)   {}", self.autom.low_disk_threshold_pct, onoff(self.autom.low_disk_watch)),
            format!(
                "service_watchdog        {}  watches: {}",
                onoff(self.autom.service_watchdog),
                if self.autom.watch_services.is_empty() { "none".into() } else { self.autom.watch_services.join(", ") }
            ),
            format!("high_cpu_alert (>={:.0}%)  {}", self.autom.cpu_alert_threshold, onoff(self.autom.high_cpu_alert)),
        ];
    }

    fn refresh_procs(&mut self) {
        self.sys.refresh_processes(ProcessesToUpdate::All, true);
        let mut v: Vec<Proc> = self
            .sys
            .processes()
            .iter()
            .map(|(pid, p)| Proc {
                pid: pid.as_u32(),
                name: p.name().to_string_lossy().into_owned(),
                state: proc_status(pid.as_u32()),
                cpu: p.cpu_usage(),
                mem: p.memory(),
            })
            .collect();
        v.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
        if !self.proc_filter.is_empty() {
            let f = self.proc_filter.to_lowercase();
            v.retain(|p| p.name.to_lowercase().contains(&f));
        }
        self.procs = v;
        if self.proc_state.selected().is_none() && !self.procs.is_empty() {
            self.proc_state.select(Some(0));
        }
        if self.autom.high_cpu_alert {
            self.alerts = self
                .procs
                .iter()
                .filter(|p| p.cpu >= self.autom.cpu_alert_threshold)
                .map(|p| format!("⚠ {} at {:.0}% CPU", p.name, p.cpu))
                .collect();
        } else {
            self.alerts.clear();
        }
    }

    fn refresh_services(&mut self) {
        if let Ok(out) = std::process::Command::new("systemctl")
            .args(["list-units", "--type=service", "--all", "--plain", "--no-legend", "--no-pager"])
            .output()
        {
            self.services = String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|l| {
                    // `splitn(.., char::is_whitespace)` splits on every single
                    // whitespace byte, so the column padding between fields
                    // yields empty strings and the description swallows the
                    // LOAD/ACTIVE/SUB columns. Split on runs instead.
                    let mut it = l.split_whitespace();
                    Some(Service {
                        unit: it.next()?.to_string(),
                        load: it.next().unwrap_or("").to_string(),
                        active: it.next().unwrap_or("").to_string(),
                        sub: it.next().unwrap_or("").to_string(),
                        description: it.collect::<Vec<_>>().join(" "),
                    })
                })
                .collect();
        }
        if self.svc_state.selected().is_none() && !self.services.is_empty() {
            self.svc_state.select(Some(0));
        }
    }

    fn refresh_disk(&mut self) {
        let disks = sysinfo::Disks::new_with_refreshed_list();
        self.disks = disks
            .iter()
            .map(|d| (d.mount_point().to_string_lossy().into_owned(), d.total_space(), d.available_space()))
            .collect();
        let home = std::env::var("HOME").unwrap_or_default();
        let mut big: Vec<(String, u64)> = vec![];
        for name in [".cache", ".var", ".local", ".config", "Dev", "Assets", "Downloads", "Documents", ".opencode", ".npm-global", ".rokit", ".vibe-kanban"] {
            let p = format!("{home}/{name}");
            let size = clean::dir_size(std::path::Path::new(&p));
            if size > 0 {
                big.push((format!("~/{name}"), size));
            }
        }
        big.sort_by(|a, b| b.1.cmp(&a.1));
        self.bigdirs = big;
    }

    fn refresh_clean(&mut self) {
        self.clean_targets = clean::targets()
            .iter()
            .map(|t| {
                let size = if t.needs_sudo {
                    0
                } else if t.name.starts_with("Flatpak") {
                    clean::flatpak_cache_size()
                } else {
                    clean::dir_size(&t.path)
                };
                (t.name.to_string(), size, false, t.needs_sudo)
            })
            .collect();
        if self.clean_state.selected().is_none() && !self.clean_targets.is_empty() {
            self.clean_state.select(Some(0));
        }
    }

    /// PIDs to act on: all marked, else the highlighted row.
    fn actionable_pids(&self) -> Vec<(u32, String)> {
        if !self.proc_marks.is_empty() {
            self.procs
                .iter()
                .filter(|p| self.proc_marks.contains(&p.pid))
                .map(|p| (p.pid, p.name.clone()))
                .collect()
        } else {
            self.proc_state
                .selected()
                .and_then(|i| self.procs.get(i))
                .map(|p| vec![(p.pid, p.name.clone())])
                .unwrap_or_default()
        }
    }

    fn signal_actionable(&mut self, sig: i32) {
        let targets = self.actionable_pids();
        if targets.is_empty() {
            self.status = "no process selected".into();
            return;
        }
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        let signal = match sig {
            1 => Signal::SIGTERM,
            2 => Signal::SIGKILL,
            3 => Signal::SIGSTOP,
            _ => Signal::SIGCONT,
        };
        let mut ok = 0;
        for (pid, _) in &targets {
            if kill(Pid::from_raw(*pid as i32), signal).is_ok() {
                ok += 1;
            }
        }
        self.status = format!("→ {signal:?} to {ok}/{} process(es)", targets.len());
        self.proc_marks.clear();
        self.refresh_procs();
    }

    fn pause_resume_actionable(&mut self) {
        let targets = self.actionable_pids();
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        for (pid, _) in targets {
            let stopped = proc_status(pid) == ProcessStatus::Stop;
            let signal = if stopped { Signal::SIGCONT } else { Signal::SIGSTOP };
            let _ = kill(Pid::from_raw(pid as i32), signal);
        }
        self.status = "pause/resume toggled".into();
        self.proc_marks.clear();
        self.refresh_procs();
    }

    fn clean_selected(&mut self) {
        let targets: Vec<String> = self
            .clean_targets
            .iter()
            .filter(|t| t.2 && !t.3)
            .map(|t| t.0.clone())
            .collect();
        if targets.is_empty() {
            let sudo_hint = self
                .clean_targets
                .iter()
                .position(|t| t.2 && t.3)
                .and_then(|i| {
                    let name = &self.clean_targets[i].0;
                    clean::targets().into_iter().find(|t| &t.name == name).and_then(|t| t.sudo_cmd.map(|s| s.to_string()))
                });
            self.status = match sudo_hint {
                Some(cmd) => format!("needs sudo — run: {cmd}"),
                None => "nothing selected (space to toggle, only user caches are removable)".into(),
            };
            return;
        }
        let mut freed = 0u64;
        for name in &targets {
            for t in clean::targets().iter().filter(|t| &t.name == name) {
                if t.name.starts_with("Flatpak") {
                    let before = clean::flatpak_cache_size();
                    clean::delete_flatpak_caches();
                    freed += before;
                } else {
                    let before = clean::dir_size(&t.path);
                    if clean::delete(&t.path).is_ok() {
                        freed += before;
                    }
                }
            }
        }
        self.status = format!("cleaned {} target(s), ~{} freed", targets.len(), clean::human(freed));
        self.refresh_clean();
    }

    fn sudo_targets_selected(&self) -> Vec<String> {
        self.clean_targets
            .iter()
            .filter(|t| t.2 && t.3)
            .filter_map(|t| {
                let name = &t.0;
                clean::targets().into_iter().find(|x| &x.name == name).and_then(|x| x.sudo_cmd.map(|s| s.to_string()))
            })
            .collect()
    }

    fn run_sudo_clean(&mut self) {
        let cmds = self.sudo_targets_selected();
        if cmds.is_empty() {
            return;
        }
        for cmd in cmds {
            // strip "sudo " prefix; -S reads password from stdin
            let args: Vec<&str> = cmd.strip_prefix("sudo ").unwrap_or(&cmd).split_whitespace().collect();
            let mut child = match std::process::Command::new("sudo")
                .arg("-S")
                .arg("-k")
                .args(&args)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    self.status = format!("sudo spawn failed: {e}");
                    return;
                }
            };
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                let _ = writeln!(stdin, "{}", self.sudo_pass);
            }
            match child.wait_with_output() {
                Ok(o) if o.status.success() => {
                    self.status = format!("ok: {cmd}");
                }
                Ok(o) => {
                    let err = String::from_utf8_lossy(&o.stderr);
                    self.status = format!("sudo failed (bad password?): {}", err.lines().last().unwrap_or(""));
                }
                Err(e) => self.status = e.to_string(),
            }
        }
        self.sudo_pass.clear();
        self.refresh_clean();
    }

    fn run_automation_on_start(&mut self) {
        let mut done = vec![];
        let home = std::env::var("HOME").unwrap_or_default();
        if self.autom.prune_yay_cache {
            let _ = clean::delete(&std::path::PathBuf::from(format!("{home}/.cache/yay")));
            done.push("yay");
        }
        if self.autom.prune_thumbnails {
            let _ = clean::delete(&std::path::PathBuf::from(format!("{home}/.cache/thumbnails")));
            done.push("thumbnails");
        }
        if self.autom.prune_mesa_shader {
            let _ = clean::delete(&std::path::PathBuf::from(format!("{home}/.cache/mesa_shader_cache")));
            done.push("mesa");
        }
        if self.autom.prune_flatpak_caches {
            clean::delete_flatpak_caches();
            done.push("flatpak");
        }
        if !done.is_empty() {
            self.status = format!("automation pruned: {}", done.join(", "));
        }
    }

    fn check_watchers(&mut self) {
        if self.last_watch_check.elapsed() < Duration::from_secs(10) {
            return;
        }
        self.last_watch_check = Instant::now();
        if self.autom.low_disk_watch {
            let disks = sysinfo::Disks::new_with_refreshed_list();
            for d in disks.iter() {
                if d.mount_point() == std::path::Path::new("/") {
                    let used_pct = 100 - (d.available_space() * 100 / d.total_space().max(1));
                    if used_pct >= self.autom.low_disk_threshold_pct as u64 {
                        let home = std::env::var("HOME").unwrap_or_default();
                        clean::delete(&std::path::PathBuf::from(format!("{home}/.cache/mesa_shader_cache"))).ok();
                        clean::delete(&std::path::PathBuf::from(format!("{home}/.cache/thumbnails"))).ok();
                        clean::delete_flatpak_caches();
                        self.status = format!("low-disk watcher: / at {used_pct}%, pruned safe caches");
                    }
                }
            }
        }
        if self.autom.service_watchdog {
            for unit in self.autom.watch_services.clone() {
                let active = std::process::Command::new("systemctl")
                    .args(["is-active", &unit])
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "active")
                    .unwrap_or(false);
                if !active {
                    let _ = std::process::Command::new("systemctl").args(["restart", &unit]).output();
                    self.status = format!("watchdog: restarted {unit}");
                }
            }
        }
    }

    fn run_command(&mut self, input: &str) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        let (cmd, args) = (parts[0].to_lowercase(), &parts[1..]);
        match cmd.as_str() {
            "kill" | "term" => {
                let pids: Vec<u32> = match args.first().and_then(|a| a.parse::<u32>().ok()) {
                    Some(p) => vec![p],
                    None => self.actionable_pids().iter().map(|(p, _)| *p).collect(),
                };
                if pids.is_empty() {
                    self.status = "usage: kill <pid>  (or space-mark rows first)".into();
                } else {
                    for p in pids {
                        self.pid_signal(p, 1);
                    }
                }
            }
            "forcekill" | "kill9" => {
                let pids: Vec<u32> = match args.first().and_then(|a| a.parse::<u32>().ok()) {
                    Some(p) => vec![p],
                    None => self.actionable_pids().iter().map(|(p, _)| *p).collect(),
                };
                if pids.is_empty() {
                    self.status = "usage: forcekill <pid>  (or space-mark rows first)".into();
                } else {
                    for p in pids {
                        self.pid_signal(p, 2);
                    }
                }
            }
            "pause" => {
                let pids: Vec<u32> = match args.first().and_then(|a| a.parse::<u32>().ok()) {
                    Some(p) => vec![p],
                    None => self.actionable_pids().iter().map(|(p, _)| *p).collect(),
                };
                if pids.is_empty() {
                    self.status = "usage: pause <pid>  (or space-mark rows first)".into();
                } else {
                    for p in pids {
                        self.pid_signal(p, 3);
                    }
                }
            }
            "resume" => {
                let pids: Vec<u32> = match args.first().and_then(|a| a.parse::<u32>().ok()) {
                    Some(p) => vec![p],
                    None => self.actionable_pids().iter().map(|(p, _)| *p).collect(),
                };
                if pids.is_empty() {
                    self.status = "usage: resume <pid>  (or space-mark rows first)".into();
                } else {
                    for p in pids {
                        self.pid_signal(p, 4);
                    }
                }
            }
            "start" | "stop" | "restart" | "enable" | "disable" => {
                if let Some(unit) = args.first() {
                    match std::process::Command::new("systemctl").args([cmd.as_str(), unit]).output() {
                        Ok(o) if o.status.success() => self.status = format!("{cmd} {unit} ok"),
                        Ok(o) => self.status = String::from_utf8_lossy(&o.stderr).trim().to_string(),
                        Err(e) => self.status = e.to_string(),
                    }
                    self.refresh_services();
                } else {
                    self.status = format!("usage: {cmd} <service>");
                }
            }
            "watch" => {
                if let Some(unit) = args.first() {
                    if !self.autom.watch_services.iter().any(|u| u == unit) {
                        self.autom.watch_services.push(unit.to_string());
                        self.persist_automation();
                    }
                    self.rebuild_autom_items();
                    self.status = format!("watching {unit}");
                }
            }
            "unwatch" => {
                if let Some(unit) = args.first() {
                    self.autom.watch_services.retain(|u| u != unit);
                    self.persist_automation();
                    self.rebuild_autom_items();
                    self.status = format!("unwatched {unit}");
                }
            }
            "clean" => {
                if args.first() == Some(&"all") {
                    for t in &mut self.clean_targets {
                        if !t.3 {
                            t.2 = true;
                        }
                    }
                    self.clean_selected();
                } else if let Some(name) = args.first() {
                    let lname = name.to_lowercase();
                    for t in &mut self.clean_targets {
                        if t.0.to_lowercase().contains(&lname) && !t.3 {
                            t.2 = true;
                        }
                    }
                    self.clean_selected();
                } else {
                    self.status = "usage: clean <target|all>".into();
                }
            }
            "filter" => {
                self.proc_filter = args.join(" ");
                self.refresh_procs();
            }
            "clear" => {
                self.proc_filter.clear();
                self.refresh_procs();
            }
            "mark" | "select" => {
                if let Some(i) = args.first().and_then(|a| a.parse::<usize>().ok()) {
                    if let Some(p) = self.procs.get(i) {
                        let pid = p.pid;
                        if !self.proc_marks.remove(&pid) {
                            self.proc_marks.insert(pid);
                        }
                        self.status = format!("toggled mark on {} ({})", p.name, self.proc_marks.len());
                    }
                }
            }
            "goto" | "tab" => {
                if let Some(name) = args.first() {
                    let n = name.to_lowercase();
                    if let Some(i) = TABS.iter().position(|t| t.to_lowercase().starts_with(&n)) {
                        self.tab = i;
                    } else if let Ok(i) = name.parse::<usize>() {
                        self.tab = i.min(TABS.len() - 1);
                    }
                }
            }
            "set" => {
                if args.len() == 2 {
                    match args[0] {
                        "cpu" => {
                            if let Ok(v) = args[1].parse::<f32>() {
                                self.autom.cpu_alert_threshold = v;
                                self.status = format!("cpu alert threshold = {v}%");
                            }
                        }
                        "disk" => {
                            if let Ok(v) = args[1].parse::<u8>() {
                                self.autom.low_disk_threshold_pct = v;
                                self.status = format!("disk threshold = {v}%");
                            }
                        }
                        _ => self.status = "set cpu <pct> | set disk <pct>".into(),
                    }
                    if !self.persist_automation() {
                        self.status = SAVE_BLOCKED.into();
                    }
                    self.rebuild_autom_items();
                }
            }
            "refresh" => {
                self.refresh_procs();
                self.refresh_services();
                self.refresh_disk();
                self.refresh_clean();
                self.status = "refreshed".into();
            }
            "help" => {
                self.status = "kill|term|forcekill|pause|resume <pid> · start|stop|restart|enable|disable <svc> · watch|unwatch <svc> · clean <target|all> · filter <t> · clear · mark <row> · goto <tab> · set cpu|disk <n> · refresh · quit".into();
            }
            "quit" | "exit" => std::process::exit(0),
            other => self.status = format!("unknown command: {other} (try 'help')"),
        }
    }

    fn run_shell(&mut self, line: &str) {
        self.shell_log.push(format!("$ {line}"));
        match std::process::Command::new("sh")
            .arg("-c")
            .arg(line)
            .output()
        {
            Ok(o) => {
                let mut out = String::from_utf8_lossy(&o.stdout).to_string();
                let err = String::from_utf8_lossy(&o.stderr).to_string();
                if !err.trim().is_empty() {
                    if !out.is_empty() {
                        out.push('\n');
                    }
                    out.push_str(&err);
                }
                // strip ANSI escape sequences + control bytes so the TUI doesn't get corrupted
                let mut clean = String::new();
                let mut it = out.chars().peekable();
                while let Some(c) = it.next() {
                    if c == '\x1b' {
                        // skip until the final byte of the sequence (a letter or '~')
                        for n in it.by_ref() {
                            if n.is_ascii_alphabetic() || n == '~' {
                                break;
                            }
                        }
                        continue;
                    }
                    if !c.is_control() || c == '\t' {
                        clean.push(c);
                    }
                }
                for l in clean.lines() {
                    self.shell_log.push(l.to_string());
                }
                if clean.lines().count() == 0 {
                    self.shell_log.push("(no output)".into());
                }
            }
            Err(e) => self.shell_log.push(format!("spawn failed: {e}")),
        }
        if self.shell_log.len() > 500 {
            let drain = self.shell_log.len() - 500;
            self.shell_log.drain(..drain);
        }
    }

    fn refresh_net(&mut self) {
        self.net_nets.refresh(true);
        let mut rx = 0u64;
        let mut tx = 0u64;
        for (_, data) in self.net_nets.iter() {
            rx += data.received();
            tx += data.transmitted();
        }
        self.net_hist.push_back((rx, tx));
        if self.net_hist.len() > 60 {
            self.net_hist.pop_front();
        }
    }

    fn execute_net(&mut self) {
        let selected: Vec<String> = self
            .net_fns
            .iter()
            .filter(|f| f.2)
            .map(|f| f.0.clone())
            .collect();
        if selected.is_empty() {
            // fall back to highlighted row
            if let Some(i) = self.net_state.selected() {
                if let Some(f) = self.net_fns.get(i) {
                    self.run_net_fn(&f.0.clone());
                }
            }
            return;
        }
        for name in selected {
            self.run_net_fn(&name);
        }
    }

    fn run_net_fn(&mut self, name: &str) {
        match name {
            "Live Speed Monitor" => {
                self.net_live = true;
                self.net_text = None;
                self.net_hist.clear();
                self.refresh_net();
            }
            "Wi-Fi Networks" => {
                self.net_text = Some(format_wifi());
            }
            "Active Connections" => {
                self.net_text = Some(run(&["ss", "-tup"]));
            }
            "Ping Gateway" => {
                let gw = gateway().unwrap_or_else(|| "1.1.1.1".into());
                let mut out = vec![format!("PING {} (4 packets, 2s timeout)", gw)];
                out.extend(run(&["ping", "-c", "4", "-W", "2", &gw]));
                self.net_text = Some(out);
            }
            "Flush DNS Cache" => {
                if std::process::Command::new("resolvectl").arg("flush-caches").output().map(|o| o.status.success()).unwrap_or(false) {
                    self.status = "flushed via resolvectl".into();
                } else if std::process::Command::new("nmcli").args(["general", "reload", "dns-full"]).output().map(|o| o.status.success()).unwrap_or(false) {
                    self.status = "flushed via nmcli (resolvectl unavailable)".into();
                } else {
                    self.status = "no flushable resolver: systemd-resolved not running and nmcli failed".into();
                }
            }
            _ => self.status = format!("unknown net function: {name}"),
        }
    }

    fn pid_signal(&mut self, pid: u32, sig: i32) {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        let signal = match sig {
            1 => Signal::SIGTERM,
            2 => Signal::SIGKILL,
            3 => Signal::SIGSTOP,
            _ => Signal::SIGCONT,
        };
        match kill(Pid::from_raw(pid as i32), signal) {
            Ok(_) => self.status = format!("→ {signal:?} to pid {pid}"),
            Err(e) => self.status = format!("failed: {e}"),
        }
        self.refresh_procs();
    }
}

fn format_wifi() -> Vec<String> {
    let out = std::process::Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,SECURITY,IN-USE", "device", "wifi", "list"])
        .output();
    let Ok(o) = out else {
        return vec!["nmcli unavailable".into()];
    };
    let mut lines = vec![format!("{:<32} {:<7} {:<12} {}", "SSID", "SIGNAL", "SECURITY", "IN-USE")];
    for l in String::from_utf8_lossy(&o.stdout).lines() {
        let mut it = l.rsplitn(4, ':');
        let in_use = it.next().unwrap_or("");
        let security = it.next().unwrap_or("");
        let signal = it.next().unwrap_or("");
        let ssid = it.next().unwrap_or("");
        lines.push(format!(
            "{:<32} {:<7} {:<12} {}",
            ssid,
            format!("{signal}%"),
            security,
            if in_use == "*" { "* connected" } else { "" }
        ));
    }
    lines
}

fn speed_phrase(bps: u64) -> (&'static str, Color) {
    const KIB: u64 = 1024;
    match bps {
        b if b < 64 * KIB => ("Low", Color::DarkGray),
        b if b < 512 * KIB => ("Medium", Color::Blue),
        b if b < 2 * 1024 * KIB => ("Medium-High", Color::Cyan),
        b if b < 8 * 1024 * KIB => ("Good", Color::Green),
        b if b < 32 * 1024 * KIB => ("Fast", Color::Yellow),
        _ => ("Excellent", Color::Magenta),
    }
}

fn gateway() -> Option<String> {
    let out = std::process::Command::new("ip").args(["route", "show", "default"]).output().ok()?;
    let s = String::from_utf8_lossy(&out.stdout);
    s.split_whitespace().nth(2).map(|s| s.to_string())
}

fn run(cmd: &[&str]) -> Vec<String> {
    match std::process::Command::new(cmd[0]).args(&cmd[1..]).output() {
        Ok(o) => {
            let mut lines: Vec<String> = String::from_utf8_lossy(&o.stdout)
                .lines()
                .map(|l| l.to_string())
                .collect();
            let err = String::from_utf8_lossy(&o.stderr);
            if !err.trim().is_empty() {
                lines.push(err.trim().to_string());
            }
            if lines.is_empty() {
                lines.push("(no output)".into());
            }
            lines
        }
        Err(e) => vec![format!("spawn failed: {e}")],
    }
}

fn onoff(b: bool) -> &'static str {
    if b { "[ON] " } else { "[off]" }
}

fn draw(f: &mut Frame, s: &mut State, body_area: &mut Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0), Constraint::Length(3), Constraint::Length(4)])
        .split(f.area());

    let banner = vec![
        Line::from(Span::styled(" ██╗   ██╗ █████╗ ███╗   ███╗", Style::default().fg(Color::Cyan).bold())),
        Line::from(Span::styled(" ██║   ██║██╔══██╗████╗ ████║", Style::default().fg(Color::Blue).bold())),
        Line::from(Span::styled(" ╚██╗ ██╔╝███████║██╔████╔██║", Style::default().fg(Color::Magenta).bold())),
        Line::from(Span::styled("  ╚████╔╝ ██╔══██║██║╚██╔╝██║  Arch supervisor", Style::default().fg(Color::Yellow).bold())),
    ];
    let hdr = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(38), Constraint::Min(0)])
        .split(outer[0]);
    f.render_widget(Paragraph::new(banner), hdr[0]);
    s.tabs_rect = hdr[1];

    let titles: Vec<Line> = TABS
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let style = if i == s.tab {
                Style::default().fg(TAB_COLORS[i]).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Line::from(Span::styled(format!(" {} ", t), style))
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" tabs (click) "))
        .select(s.tab);
    f.render_widget(tabs, hdr[1]);

    let body = outer[1];
    *body_area = body;
    match s.tab {
        0 => draw_procs(f, s, body),
        1 => draw_services(f, s, body),
        2 => draw_disk(f, s, body),
        3 => draw_clean(f, s, body),
        4 => draw_autom(f, s, body),
        5 => draw_shell(f, s, body),
        _ => draw_network(f, s, body),
    }

    let cmd_text = match s.mode {
        Mode::Command => format!(":{}▏", s.cmd),
        Mode::Sudo => format!("sudo password: {}", "*".repeat(s.sudo_pass.len())),
        Mode::Normal => {
            if s.tab == 5 {
                ": (press ':' to type a shell command)".into()
            } else {
                ": (press ':' to enter a command — help for list)".into()
            }
        }
    };
    let cmd_style = match s.mode {
        Mode::Command => Style::default().fg(Color::Yellow),
        Mode::Sudo => Style::default().fg(Color::Red),
        Mode::Normal => Style::default().fg(Color::DarkGray),
    };
    let prompt = if s.tab == 5 { " shell> " } else { " command " };
    f.render_widget(
        Paragraph::new(cmd_text).style(cmd_style).block(Block::default().borders(Borders::ALL).title(prompt)),
        outer[2],
    );

    let mut status_lines = vec![Line::from(s.status.clone())];
    if !s.alerts.is_empty() {
        status_lines.push(Line::from(Span::styled(s.alerts.join("  |  "), Style::default().fg(Color::Red).bold())));
    }
    let help = "Tab:switch  j/k:move  space:mark  x:term  K/d:kill9  p:pause  ::command  q:quit";
    f.render_widget(
        Paragraph::new(status_lines).block(Block::default().borders(Borders::ALL).title(help)).wrap(Wrap { trim: true }),
        outer[3],
    );
}

fn draw_procs(f: &mut Frame, s: &mut State, area: Rect) {
    let total = s.procs.len();
    let running = s.procs.iter().filter(|p| p.state == ProcessStatus::Run).count();
    let stopped = s.procs.iter().filter(|p| p.state == ProcessStatus::Stop).count();
    let marked = s.proc_marks.len();
    let rows: Vec<Row> = s
        .procs
        .iter()
        .map(|p| {
            let (label, color) = proc_state_label(&p.state);
            let mark = if s.proc_marks.contains(&p.pid) { "[×]" } else { "[ ]" };
            Row::new(vec![
                Cell::from(mark),
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(label).style(Style::default().fg(color)),
                Cell::from(format!("{:.1}", p.cpu)).style(if p.cpu > 50.0 { Style::default().fg(Color::Red) } else { Style::default() }),
                Cell::from(clean::human(p.mem)),
            ])
        })
        .collect();
    let widths = [Constraint::Length(4), Constraint::Length(8), Constraint::Min(20), Constraint::Length(11), Constraint::Length(7), Constraint::Length(10)];
    let title = if s.proc_filter.is_empty() {
        format!(" processes: {total}  running: {running}  stopped: {stopped}  marked: {marked} ")
    } else {
        format!(" processes (filter: {}) — {total} ", s.proc_filter)
    };
    let table = Table::new(rows, widths)
        .header(Row::new(vec!["", "PID", "NAME", "STATE", "CPU%", "MEM"]).style(Style::default().fg(Color::Cyan).bold()))
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title(title).border_style(Color::Cyan));
    f.render_stateful_widget(table, area, &mut s.proc_state);
}

fn draw_services(f: &mut Frame, s: &mut State, area: Rect) {
    let active_n = s.services.iter().filter(|v| v.active == "active").count();
    let rows: Vec<Row> = s
        .services
        .iter()
        .map(|v| {
            let style = if v.active == "active" { Style::default().fg(Color::Green) } else { Style::default().fg(Color::DarkGray) };
            Row::new(vec![
                Cell::from(v.unit.clone()),
                Cell::from(v.load.clone()),
                Cell::from(v.active.clone()).style(style),
                Cell::from(v.sub.clone()),
                Cell::from(v.description.clone()),
            ])
        })
        .collect();
    let widths = [Constraint::Length(30), Constraint::Length(6), Constraint::Length(9), Constraint::Length(9), Constraint::Min(20)];
    let table = Table::new(rows, widths)
        .header(Row::new(vec!["UNIT", "LOAD", "ACTIVE", "SUB", "DESCRIPTION"]).style(Style::default().fg(Color::Green).bold()))
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title(format!(" services: {} active of {} ", active_n, s.services.len())).border_style(Color::Green));
    f.render_stateful_widget(table, area, &mut s.svc_state);
}

fn draw_disk(f: &mut Frame, s: &State, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(9), Constraint::Min(0)])
        .split(area);
    let rows: Vec<Row> = s
        .disks
        .iter()
        .map(|(mount, total, avail)| {
            let used = total - avail;
            let pct = 100 - (avail * 100 / (*total).max(1));
            let color = if pct > 90 { Color::Red } else if pct > 70 { Color::Yellow } else { Color::Green };
            Row::new(vec![
                Cell::from(mount.clone()),
                Cell::from(clean::human(*total)),
                Cell::from(clean::human(used)),
                Cell::from(format!("{}%", pct)).style(Style::default().fg(color)),
            ])
        })
        .collect();
    let widths = [Constraint::Min(8), Constraint::Length(10), Constraint::Length(10), Constraint::Length(6)];
    let t = Table::new(rows, widths)
        .header(Row::new(vec!["MOUNT", "TOTAL", "USED", "USE%"]).style(Style::default().fg(Color::Yellow).bold()))
        .block(Block::default().borders(Borders::ALL).title(" mounts ").border_style(Color::Yellow));
    f.render_widget(t, chunks[0]);

    let items: Vec<ListItem> = s
        .bigdirs
        .iter()
        .map(|(name, size)| ListItem::new(format!("{name:<22} {}", clean::human(*size))).style(Style::default().fg(Color::Yellow)))
        .collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" top dirs in $HOME ").border_style(Color::Yellow));
    f.render_widget(list, chunks[1]);
}

fn draw_clean(f: &mut Frame, s: &mut State, area: Rect) {
    let rows: Vec<Row> = s
        .clean_targets
        .iter()
        .map(|(name, size, sel, sudo)| {
            Row::new(vec![
                Cell::from(if *sel { "[×]" } else { "[ ]" }).style(if *sel { Style::default().fg(Color::Green) } else { Style::default() }),
                Cell::from(name.clone()),
                Cell::from(if *sudo { "—".into() } else { clean::human(*size) }).style(Style::default().fg(Color::Magenta)),
                Cell::from(if *sudo { "sudo" } else { "user" }).style(if *sudo { Style::default().fg(Color::Red) } else { Style::default().fg(Color::Green) }),
            ])
        })
        .collect();
    let widths = [Constraint::Length(5), Constraint::Min(20), Constraint::Length(12), Constraint::Length(6)];
    let table = Table::new(rows, widths)
        .header(Row::new(vec!["", "TARGET", "SIZE", "NEEDS"]).style(Style::default().fg(Color::Magenta).bold()))
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title(" cleaner ").border_style(Color::Magenta));
    f.render_stateful_widget(table, area, &mut s.clean_state);

    if s.confirm_clean {
        let area = centered_rect(50, 20, area);
        f.render_widget(Clear, area);
        let p = Paragraph::new("Delete selected user caches?\nEnter: yes   Esc: no")
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title(" confirm ").border_style(Color::Red))
            .alignment(Alignment::Center);
        f.render_widget(p, area);
    }
}

fn draw_autom(f: &mut Frame, s: &mut State, area: Rect) {
    let items: Vec<ListItem> = s
        .autom_items
        .iter()
        .enumerate()
        .map(|(i, it)| {
            let on = it.contains("[ON]");
            let style = if on { Style::default().fg(Color::Green) } else { Style::default().fg(Color::DarkGray) };
            let prefix = if s.autom_state.selected() == Some(i) { "▶ " } else { "  " };
            ListItem::new(format!("{prefix}{it}")).style(style)
        })
        .collect();
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" automations — space toggles · persisted to ~/.config/vam/config.toml ")
            .border_style(Color::Blue),
    );
    f.render_widget(list, area);
}

fn draw_network(f: &mut Frame, s: &mut State, area: Rect) {
    if s.net_live {
        // live metrics sub-view
        let max_rate = s.net_hist.iter().map(|(r, t)| (*r).max(*t)).max().unwrap_or(1).max(1);
        let spark = |i: usize| {
            let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
            s.net_hist
                .iter()
                .map(|(rx, tx)| {
                    let v = if i == 0 { *rx } else { *tx };
                    let idx = ((v as f64 / max_rate as f64) * 7.0) as usize;
                    blocks[idx.min(7)]
                })
                .collect::<String>()
        };
        let (rx_now, tx_now) = s.net_hist.back().copied().unwrap_or((0, 0));
        let total_rx: u64 = s.net_nets.iter().map(|(_, d)| d.total_received()).sum();
        let total_tx: u64 = s.net_nets.iter().map(|(_, d)| d.total_transmitted()).sum();
        let (rx_phrase, rx_color) = speed_phrase(rx_now);
        let (tx_phrase, tx_color) = speed_phrase(tx_now);

        let mut lines = vec![
            Line::from(Span::styled("━━ LIVE NETWORK ━━", Style::default().fg(Color::White).bold())),
            Line::from(""),
            Line::from(vec![
                Span::styled("▼ DOWN ", Style::default().fg(Color::Green).bold()),
                Span::styled(format!("{:>12}/s", clean::human(rx_now)), Style::default().fg(Color::Green).bold()),
                Span::styled(format!("  {rx_phrase}"), Style::default().fg(rx_color).bold()),
            ]),
            Line::from(spark(0)).style(Style::default().fg(Color::Green)),
            Line::from(""),
            Line::from(vec![
                Span::styled("▲ UP   ", Style::default().fg(Color::Blue).bold()),
                Span::styled(format!("{:>12}/s", clean::human(tx_now)), Style::default().fg(Color::Blue).bold()),
                Span::styled(format!("  {tx_phrase}"), Style::default().fg(tx_color).bold()),
            ]),
            Line::from(spark(1)).style(Style::default().fg(Color::Blue)),
            Line::from(""),
            Line::from(vec![
                Span::styled(format!("total rx {:>12}   tx {:>12}", clean::human(total_rx), clean::human(total_tx)), Style::default().fg(Color::White)),
            ]),
            Line::from(""),
        ];
        lines.push(Line::from(Span::styled(" interfaces ", Style::default().fg(Color::Yellow).bold())));
        for (name, d) in s.net_nets.iter() {
            lines.push(Line::from(format!(
                "{name:<12} rx {:>10}   tx {:>10}",
                clean::human(d.total_received()),
                clean::human(d.total_transmitted())
            )));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("Enter / Esc: back to function list", Style::default().fg(Color::DarkGray))));
        f.render_widget(
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Network — live ").border_style(Color::White)),
            area,
        );
        return;
    }

    if let Some(text) = &s.net_text {
        let lines: Vec<Line> = text.iter().map(|l| Line::from(l.clone())).collect();
        f.render_widget(
            Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title(" Network — output (Enter/Esc: back) ").border_style(Color::White))
                .wrap(Wrap { trim: false }),
            area,
        );
        return;
    }

    let rows: Vec<Row> = s
        .net_fns
        .iter()
        .map(|(name, desc, sel)| {
            Row::new(vec![
                Cell::from(if *sel { "[×]" } else { "[ ]" }).style(if *sel { Style::default().fg(Color::Green) } else { Style::default() }),
                Cell::from(name.clone()),
                Cell::from(desc.clone()),
            ])
        })
        .collect();
    let widths = [Constraint::Length(5), Constraint::Length(22), Constraint::Min(20)];
    let table = Table::new(rows, widths)
        .header(Row::new(vec!["", "FUNCTION", "DESCRIPTION"]).style(Style::default().fg(Color::White).bold()))
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title(" network functions — space to mark · Enter runs ").border_style(Color::White));
    f.render_stateful_widget(table, area, &mut s.net_state);
}

fn draw_shell(f: &mut Frame, s: &State, area: Rect) {
    let lines: Vec<Line> = s
        .shell_log
        .iter()
        .rev()
        .take(area.height.saturating_sub(2) as usize)
        .rev()
        .map(|l| {
            if l.starts_with('$') {
                Line::from(Span::styled(l.clone(), Style::default().fg(Color::Yellow)))
            } else {
                Line::from(l.clone())
            }
        })
        .collect();
    let p = Paragraph::new(lines).block(
        Block::default().borders(Borders::ALL).title(" shell — ':' then type a command ").border_style(Color::Red),
    );
    f.render_widget(p, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage((100 - percent_y) / 2), Constraint::Percentage(percent_y), Constraint::Percentage((100 - percent_y) / 2)])
        .split(r)[1];
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage((100 - percent_x) / 2), Constraint::Percentage(percent_x), Constraint::Percentage((100 - percent_x) / 2)])
        .split(popup)[1]
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    // Restore unconditionally: `?` inside run_app would otherwise unwind past
    // this point and leave the user with raw mode + alt screen still active.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
    terminal.show_cursor()?;
    result
}

fn run_app(terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
    let mut s = State::new();
    s.run_automation_on_start();
    let mut body_area = Rect::default();

    let mut last_refresh = Instant::now();
    loop {
        terminal.draw(|f| draw(f, &mut s, &mut body_area))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if s.confirm_clean {
                        match key.code {
                            KeyCode::Enter => {
                                s.confirm_clean = false;
                                if s.sudo_targets_selected().is_empty() {
                                    s.clean_selected();
                                } else {
                                    s.mode = Mode::Sudo;
                                    s.sudo_pass.clear();
                                }
                            }
                            _ => s.confirm_clean = false,
                        }
                        continue;
                    }
                    match s.mode {
                        Mode::Command => match key.code {
                            KeyCode::Enter => {
                                let input = s.cmd.clone();
                                s.cmd.clear();
                                s.mode = Mode::Normal;
                                if s.tab == 5 {
                                    s.run_shell(&input);
                                } else {
                                    s.run_command(&input);
                                }
                            }
                            KeyCode::Esc => {
                                s.cmd.clear();
                                s.mode = Mode::Normal;
                            }
                            KeyCode::Backspace => {
                                s.cmd.pop();
                            }
                            KeyCode::Char(c) => s.cmd.push(c),
                            _ => {}
                        },
                        Mode::Sudo => match key.code {
                            KeyCode::Enter => {
                                s.mode = Mode::Normal;
                                // Must run before run_sudo_clean: that calls
                                // refresh_clean(), which resets every mark.
                                // clean_selected() skips the sudo rows itself.
                                s.clean_selected();
                                s.run_sudo_clean();
                            }
                            KeyCode::Esc => {
                                s.sudo_pass.clear();
                                s.mode = Mode::Normal;
                                s.status = "sudo cancelled".into();
                            }
                            KeyCode::Backspace => {
                                s.sudo_pass.pop();
                            }
                            KeyCode::Char(c) => s.sudo_pass.push(c),
                            _ => {}
                        },
                        Mode::Normal => match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char(':') => {
                                s.mode = Mode::Command;
                                s.cmd.clear();
                            }
                            KeyCode::Tab => s.tab = (s.tab + 1) % TABS.len(),
                            KeyCode::BackTab => s.tab = (s.tab + TABS.len() - 1) % TABS.len(),
                            KeyCode::Char('r') => {
                                s.refresh_procs();
                                s.refresh_services();
                                s.refresh_disk();
                                s.refresh_clean();
                            }
                            KeyCode::Down | KeyCode::Char('j') => move_sel(&mut s, 1),
                            KeyCode::Up | KeyCode::Char('k') => move_sel(&mut s, -1),
                            KeyCode::Home | KeyCode::Char('g') => move_sel_to(&mut s, 0),
                            KeyCode::End | KeyCode::Char('G') => move_sel_to(&mut s, i32::MAX),
                            KeyCode::Char('x') => s.signal_actionable(1),
                            KeyCode::Char('K') | KeyCode::Char('d') => s.signal_actionable(2),
                            KeyCode::Char('p') => s.pause_resume_actionable(),
                            KeyCode::Char(' ') => match s.tab {
                                0 => {
                                    if let Some(i) = s.proc_state.selected() {
                                        if let Some(p) = s.procs.get(i) {
                                            let pid = p.pid;
                                            if !s.proc_marks.remove(&pid) {
                                                s.proc_marks.insert(pid);
                                            }
                                        }
                                    }
                                }
                                3 => {
                                    if let Some(i) = s.clean_state.selected() {
                                        if let Some(t) = s.clean_targets.get_mut(i) {
                                            t.2 = !t.2;
                                        }
                                    }
                                }
                                4 => {
                                    if let Some(i) = s.autom_state.selected() {
                                        let mut a = s.autom.clone();
                                        match i {
                                            0 => a.prune_yay_cache = !a.prune_yay_cache,
                                            1 => a.prune_thumbnails = !a.prune_thumbnails,
                                            2 => a.prune_mesa_shader = !a.prune_mesa_shader,
                                            3 => a.prune_flatpak_caches = !a.prune_flatpak_caches,
                                            4 => a.low_disk_watch = !a.low_disk_watch,
                                            5 => a.service_watchdog = !a.service_watchdog,
                                            _ => a.high_cpu_alert = !a.high_cpu_alert,
                                        }
                                        s.autom = a;
                                        let saved = s.persist_automation();
                                        s.rebuild_autom_items();
                                        s.status = if saved { "automation saved".into() } else { SAVE_BLOCKED.into() };
                                    }
                                }
                                6 => {
                                    if let Some(i) = s.net_state.selected() {
                                        // radio behaviour: only one marked at a time
                                        let was = s.net_fns[i].2;
                                        for f in s.net_fns.iter_mut() {
                                            f.2 = false;
                                        }
                                        s.net_fns[i].2 = !was;
                                    }
                                }
                                _ => {}
                            },
                            KeyCode::Enter => match s.tab {
                                1 => {
                                    if let Some(i) = s.svc_state.selected() {
                                        if let Some(v) = s.services.get(i) {
                                            let action = if v.active == "active" { "stop" } else { "start" };
                                            match std::process::Command::new("systemctl").args([action, &v.unit]).output() {
                                                Ok(o) if o.status.success() => s.status = format!("{action} {}", v.unit),
                                                Ok(o) => s.status = String::from_utf8_lossy(&o.stderr).trim().to_string(),
                                                Err(e) => s.status = e.to_string(),
                                            }
                                            s.refresh_services();
                                        }
                                    }
                                }
                                3 => s.confirm_clean = true,
                                5 => s.mode = Mode::Command,
                                6 => {
                                    if s.net_live || s.net_text.is_some() {
                                        s.net_live = false;
                                        s.net_text = None;
                                    } else {
                                        s.execute_net();
                                    }
                                }
                                _ => {}
                            },
                            KeyCode::Esc if s.tab == 6 && (s.net_live || s.net_text.is_some()) => {
                                s.net_live = false;
                                s.net_text = None;
                            }
                            KeyCode::Char('e') if s.tab == 1 => {
                                if let Some(i) = s.svc_state.selected() {
                                    if let Some(v) = s.services.get(i) {
                                        let _ = std::process::Command::new("systemctl").args(["enable", &v.unit]).output();
                                        s.status = format!("enabled {}", v.unit);
                                        s.refresh_services();
                                    }
                                }
                            }
                            _ => {}
                        },
                    }
                }
                Event::Mouse(MouseEvent { kind, column, row, .. }) => match kind {
                    MouseEventKind::ScrollDown => move_sel(&mut s, 1),
                    MouseEventKind::ScrollUp => move_sel(&mut s, -1),
                    MouseEventKind::Down(MouseButton::Left) => {
                        let tr = s.tabs_rect;
                        if row >= tr.y + 1 && row < tr.y + tr.height.saturating_sub(1) {
                            // click in tab header: walk exact title widths
                            if column >= tr.x {
                                let mut acc = tr.x + 1; // left border
                                for (i, t) in TABS.iter().enumerate() {
                                    let w = (t.len() + 2) as u16; // " Title "
                                    if column >= acc && column < acc + w {
                                        s.tab = i;
                                        break;
                                    }
                                    acc += w;
                                }
                            }
                        } else if row > body_area.y && row < body_area.y + body_area.height.saturating_sub(1) {
                            // Rows start after the block border; the table tabs
                            // add a header row, the Automation List does not.
                            let header_rows = if s.tab == 4 { 1 } else { 2 };
                            let idx = row.saturating_sub(body_area.y + header_rows) as usize;
                            match s.tab {
                                0 if idx < s.procs.len() => s.proc_state.select(Some(idx)),
                                1 if idx < s.services.len() => s.svc_state.select(Some(idx)),
                                3 if idx < s.clean_targets.len() => s.clean_state.select(Some(idx)),
                                4 if idx < s.autom_items.len() => s.autom_state.select(Some(idx)),
                                6 if idx < s.net_fns.len() => s.net_state.select(Some(idx)),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if last_refresh.elapsed() > Duration::from_secs(2) && s.tab == 0 {
            s.refresh_procs();
            last_refresh = Instant::now();
        }
        if s.net_live && s.net_last.elapsed() >= Duration::from_secs(1) {
            s.refresh_net();
            s.net_last = Instant::now();
        }
        s.check_watchers();
    }

    Ok(())
}

fn move_sel_to(s: &mut State, pos: i32) {
    let len = match s.tab {
        0 => s.procs.len(),
        1 => s.services.len(),
        3 => s.clean_targets.len(),
        4 => s.autom_items.len(),
        6 => s.net_fns.len(),
        _ => return,
    };
    let idx = if pos < 0 { 0 } else { (pos as usize).min(len.saturating_sub(1)) };
    match s.tab {
        0 => s.proc_state.select(Some(idx)),
        1 => s.svc_state.select(Some(idx)),
        3 => s.clean_state.select(Some(idx)),
        4 => s.autom_state.select(Some(idx)),
        _ => s.net_state.select(Some(idx)),
    }
}

fn move_sel(s: &mut State, delta: i32) {
    let (state, len) = match s.tab {
        0 => (&mut s.proc_state, s.procs.len()),
        1 => (&mut s.svc_state, s.services.len()),
        3 => (&mut s.clean_state, s.clean_targets.len()),
        4 => (&mut s.autom_state, s.autom_items.len()),
        6 => (&mut s.net_state, s.net_fns.len()),
        _ => return,
    };
    // An empty list has no valid index; `clamp(0, len - 1)` would panic on
    // `min > max` when len == 0.
    if len == 0 {
        return;
    }
    let i = match state.selected() {
        Some(i) => (i as i32 + delta).clamp(0, len as i32 - 1) as usize,
        None => 0,
    };
    state.select(Some(i));
}
