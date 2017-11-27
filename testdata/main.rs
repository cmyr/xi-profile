use std::process;
use std::thread;
use std::time::{Instant, Duration};
use std::io::{BufReader, Write, BufRead};
use std::sync::{Mutex, Arc, Barrier};
use std::borrow::Cow;
use std::fmt;

fn main() {
    let core_path = "./xi-editor/rust/target/release/xi-core";
    let mut core = process::Command::new(&core_path)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("xi-core must start");

    let mut stdin = core.stdin.take().unwrap();
    let stdout = core.stdout.take().unwrap();

    let all_results = Arc::new(Mutex::new(Vec::new()));
    let all_results2 = all_results.clone();

    let barrier = Arc::new(Barrier::new(2));
    let barrier2 = barrier.clone();

    let read_handle = thread::spawn(move || {
        let child_out = BufReader::new(stdout);
        let mut received = Vec::new();
        barrier2.wait();
        for line in child_out.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => panic!("io error, aborting: {:?}", e),
            };
            received.push((Instant::now(), Cow::from(line)));
        }

        let mut all_results = all_results2.lock().unwrap();
        all_results.append(&mut received);
    });

    let mut inp_json = vec![
        r#"{"method":"client_started","params":{}}
"#,
        r#"{"method":"set_theme","params":{"theme_name":"InspiredGitHub"}}
"#,
        r#"{"id":0,"method":"new_view","params":{}}
"#,
    ];

    let mut local_results = Vec::new();
    barrier.wait();
    let start = Instant::now();

    for line in inp_json.drain(..) {
        stdin.write_all(line.as_bytes()).expect("write failed");
        local_results.push((Instant::now(), Cow::from(line)));
    }

    {
        let mut all_results = all_results.lock().unwrap();
        all_results.append(&mut local_results);
    }

    eprintln!("killing child");
    thread::sleep(Duration::from_millis(100));
    core.kill().unwrap();
    eprintln!("joining read thread");
    read_handle.join().unwrap();
    let all_results = Arc::try_unwrap(all_results).unwrap();
    let all_results = all_results.into_inner().unwrap();

    format_results(start, all_results);
}


fn format_results(start: Instant, results: Vec<(Instant, Cow<str>)>) {
    let mut results = results.iter()
        .map(|&(i, ref r)| (i.duration_since(start), r.clone()))
        .collect::<Vec<_>>();
    results.as_mut_slice().sort_by_key(|&(i, _)| i);
    for &(d, ref r) in results.iter() {
        eprintln!("{}\t{}", FormattedTime::new(d), r.trim());
    }
}

fn nanos_from_duration(d: Duration) -> u64 {
    d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64
}

struct FormattedTime {
    secs: u64,
    millis: u64,
    micros: u64,
    nanos: u64,
}

impl FormattedTime {
    pub fn new(d: Duration) -> Self {
        let d = nanos_from_duration(d);
        let secs = d / 1_000_000_000;
        let d = d - secs;
        let millis = d / 1_000_000;
        let d = d - millis;
        let micros = d / 1_000;
        let nanos = d - micros;
        FormattedTime { secs, millis, micros, nanos }
    }
}

impl fmt::Display for FormattedTime {
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

//pub fn start_plugin_process<C>(manager_ref: &PluginManagerRef,
                          //plugin_desc: &PluginDescription,
                          //identifier: PluginPid,
                          //completion: C)
    //where C: FnOnce(Result<PluginRef, io::Error>) + Send + 'static
//{

    //let manager_ref = manager_ref.to_weak();
    //let plugin_desc = plugin_desc.to_owned();

    //thread::spawn(move || {
        //eprintln!("starting plugin at path {:?}", &plugin_desc.exec_path);
        //let child = ProcCommand::new(&plugin_desc.exec_path)
            //.stdin(Stdio::piped())
            //.stdout(Stdio::piped())
            //.spawn();

        //match child {
            //Ok(mut child) => {
                //let child_stdin = child.stdin.take().unwrap();
                //let child_stdout = child.stdout.take().unwrap();
                //let mut looper = RpcLoop::new(child_stdin);
                //let peer: RpcPeer = Box::new(looper.get_raw_peer());
                //peer.send_rpc_notification("ping", &Value::Array(Vec::new()));
                //let plugin = Plugin {
                    //peer: peer,
                    //process: child,
                    //manager: manager_ref,
                    //description: plugin_desc,
                    //identifier: identifier,
                //};
                //let mut plugin_ref = PluginRef(
                    //Arc::new(Mutex::new(plugin)),
                    //Arc::new(AtomicBool::new(false)));
                //completion(Ok(plugin_ref.clone()));
                ////TODO: we could be logging plugin exit results
                //let _ = looper.mainloop(|| BufReader::new(child_stdout),
                                        //&mut plugin_ref);
            //}
            //Err(err) => completion(Err(err)),
        //}
    //});

