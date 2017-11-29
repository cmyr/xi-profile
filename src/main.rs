extern crate xi_rpc;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate sys_info;

use std::process;
use std::thread;
use std::time::{Instant, Duration};
use std::io::{BufReader, Write, BufRead};
use std::sync::{Mutex, Arc, Barrier};
use std::borrow::Cow;
use std::fmt;

use chrono::prelude::*;
use serde_json::Value;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let cpu_num = sys_info::cpu_num().unwrap_or_default();
    let cpu_speed = sys_info::cpu_speed().unwrap_or_default();
    let os_type = sys_info::os_type().unwrap_or("".into());
    let os_release = sys_info::os_release().unwrap_or("".into());
    let load_avg = match sys_info::loadavg() {
        Ok(load) => format!("{:.2}/{:.2}/{:.2}", load.one, load.five, load.fifteen),
        Err(_) => "unavailable".to_owned(),
    };

    let mem_info = match sys_info::mem_info() {
        Ok(mem) => format!("{:?}", mem),
        Err(_) => "mem_info unavailable".to_owned(),
    };

    println!("{}", utc.to_string());
    println!("{} v{}", os_type, os_release);
    println!("cpu: {} @ {}MHz, load: {}", cpu_num, cpu_speed, load_avg);
    println!("{}", mem_info);
    println!("\n###timestamped init###\n");
    time_init();
    println!("\n###sync roundtrip###\n");
    peer_profile();
}


/// Runs a typical startup sequence, logging the time (relative to
/// the first message send) of each RPC sent and received.
///
/// This also runs the syntect plugin.
fn time_init() {

    let (mut stdin, stdout, _) = setup_core();
    let all_results = Arc::new(Mutex::new(Vec::new()));
    let all_results2 = all_results.clone();

    let barrier = Arc::new(Barrier::new(2));
    let barrier2 = barrier.clone();

    let read_handle = thread::spawn(move || {
        let child_out = BufReader::new(stdout);
        let mut received = Vec::new();
        barrier2.wait();
        for line in child_out.lines() {
            let line = line.unwrap();
            received.push((Instant::now(), "<-", Cow::from(line)));
        }

        let mut all_results = all_results2.lock().unwrap();
        all_results.append(&mut received);
    });

    let mut inp_json = vec![
"{\"method\":\"client_started\",\"params\":{\"client_extras_dir\":\"plugins\"}}\n",
"{\"id\":0,\"method\":\"new_view\",\"params\":{\"file_path\":\"./testdata/main.rs\"}}\n",
"{\"method\":\"edit\",\"params\":{\"view_id\":\"view-id-1\",\"method\":\"scroll\",\"params\":[0,100]}}\n",
"{\"method\":\"edit\",\"params\":{\"view_id\":\"view-id-1\",\"method\":\"request_lines\",\"params\":[0,100]}}\n",
    ];

    let mut local_results = Vec::new();

    // let core setup
    thread::sleep(Duration::from_millis(1000));
    barrier.wait();
    let start = Instant::now();

    for line in inp_json.drain(..) {
        stdin.write_all(line.as_bytes()).expect("write failed");
        local_results.push((Instant::now(), "->", Cow::from(line)));
    }

    // give core time to quiet down before killing
    thread::sleep(Duration::from_millis(1000));
    stdin.write_all("null\n".as_bytes()).unwrap();

    {
        let mut all_results = all_results.lock().unwrap();
        all_results.append(&mut local_results);
    }

    read_handle.join().unwrap();

    let all_results = Arc::try_unwrap(all_results).unwrap();
    let all_results = all_results.into_inner().unwrap();

    format_results(start, all_results);
}

/// Spawns a local rpc runloop, and from another thread sends a bunch
/// of 'new_view' requests to xi-core.
fn peer_profile() {
    let num_runs: u32 = 100;

    let (stdin, stdout, mut core) = setup_core();
    let mut looper = xi_rpc::RpcLoop::new(stdin);
    let peer: xi_rpc::RpcPeer = Box::new(looper.get_raw_peer());
    let b1_1 = Arc::new(Barrier::new(2));
    let b1_2 = b1_1.clone();
    thread::spawn(move || {
        b1_1.wait();
        peer.send_rpc_notification("client_started", &json!({}));
        let mut results = Vec::new();

        for _ in 0..num_runs {
            thread::sleep(Duration::from_millis(10));
            let send = Instant::now();
            let _ = peer.send_rpc_request("new_view", &json!({}));
            let duration = send.elapsed();
            results.push(duration);
        }
        let mean: Duration = results.iter().sum();
        let mean = mean / num_runs;
        let min = results.iter().min().unwrap().to_owned();
        let max = results.iter().max().unwrap().to_owned();
        println!("ran {} sync RPC requests:", num_runs);
        println!("mean: {}", PrettyDuration::new(mean));
        println!("min: {}", PrettyDuration::new(min));
        println!("max: {}", PrettyDuration::new(max));
        core.kill().unwrap();
    });

    let mut handler = ProfileHandler::default();
    b1_2.wait();
    let _ = looper.mainloop(|| BufReader::new(stdout), &mut handler);
}

fn setup_core() -> (process::ChildStdin, process::ChildStdout, process::Child) {
    let core_path = "./xi-editor/rust/target/release/xi-core";
    let mut core = process::Command::new(&core_path)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::null())
        .spawn()
        .expect("xi-core must start");

    let stdin = core.stdin.take().unwrap();
    let stdout = core.stdout.take().unwrap();
    (stdin, stdout, core)
}

fn format_results(start: Instant, results: Vec<(Instant, &'static str, Cow<str>)>) {
    let max_width = 80;
    let mut results = results.iter()
        .map(|&(i, s, ref r)| (i.duration_since(start), s, r.clone()))
        .collect::<Vec<_>>();
    results.as_mut_slice().sort_by_key(|&(i, _, _)| i);
    for &(d, s, ref r) in results.iter() {
        let mut r = r.trim().to_owned();
        if r.len() > max_width + 3 {
            r.truncate(max_width);
            r.push_str("...");
        }
        let t = format!("{}", PrettyDuration::new(d));
        println!("{:10} {}  {}", t, s, r);
    }
}

fn nanos_from_duration(d: Duration) -> u64 {
    d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64
}

struct PrettyDuration {
    secs: u64,
    millis: u64,
    micros: u64,
    nanos: u64,
}

impl PrettyDuration {
    pub fn new(d: Duration) -> Self {
        let d = nanos_from_duration(d);
        let secs = d / 1_000_000_000;
        let d = d - secs * 1_000_000_000;
        let millis = d / 1_000_000;
        let d = d - millis * 1_000_000;
        let micros = d / 1_000;
        let nanos = d - micros * 1_000;
        PrettyDuration { secs, millis, micros, nanos }
    }
}

impl fmt::Display for PrettyDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.secs > 0 {
            write!(f, "{}.{}s", self.secs, self.millis / 100)
        } else if self.millis > 0 {
            write!(f, "{}.{}ms", self.millis, self.micros / 100)
        } else if self.micros > 0 {
            write!(f, "{}us", self.micros)
        } else {
            write!(f, "{}ns", self.nanos)
        }
    }
}

//NOTE: this isn't really used right now
#[derive(Debug, Clone, Default)]
struct ProfileHandler {
    events: Vec<(Instant, String)>,
}

impl xi_rpc::Handler for ProfileHandler {
    type Notification = xi_rpc::RpcCall;
    type Request = xi_rpc::RpcCall;

    fn handle_notification(&mut self, _ctx: &xi_rpc::RpcCtx, rpc: Self::Notification) {
        self.events.push((Instant::now(), rpc.method));
    }

    fn handle_request(&mut self, _ctx: &xi_rpc::RpcCtx, rpc: Self::Request)
                      -> Result<Value, xi_rpc::RemoteError> {
        self.events.push((Instant::now(), rpc.method));
        Ok(json!(1))
    }
}

