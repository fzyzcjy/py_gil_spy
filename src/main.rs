use py_spy::python_process_info::{
    get_interpreter_address, get_python_version, get_threadstate_address, PythonProcessInfo,
};
use py_spy::stack_trace::get_gil_threadid;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2);
    let pid: i32 = args[1].parse()?;

    print_python_stacks(pid)?;

    let config = py_spy::Config::default();
    let process = remoteprocess::Process::new(pid)?;
    let python_info = PythonProcessInfo::new(&process)?;
    let version = get_python_version(&python_info, &process)?;
    let interpreter_address = get_interpreter_address(&python_info, &process, &version)?;
    println!("main start pid={pid} interpreter_address={interpreter_address}");

    let mut outputs = vec![];
    outputs.push("timestamp,gil_thread_id".to_owned());

    for _ in 0..20000 {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();

        let threadstate_address =
            get_threadstate_address(interpreter_address, &python_info, &version, &config)?;

        // ref: get_stack_traces
        // NOTE: the original code also checks `interpreter.gil_locked()`, but we skip here
        // NOTE: hack - we have 3.10 in our env
        let gil_thread_id = get_gil_threadid::<py_spy::python_bindings::v3_10_0::_is, _>(
            threadstate_address,
            &process,
        )?;
        outputs.push(format!("{},{}", timestamp, gil_thread_id));

        sleep(Duration::from_micros(1000));
    }

    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let file_path = format!("/host_home/temp_sglang_server2local/gil_spy_{}.csv", timestamp);
    println!("Write to {file_path}");
    std::fs::write(&file_path, outputs.join("\n"))?;

    Ok(())
}

fn print_python_stacks(pid: py_spy::Pid) -> Result<(), anyhow::Error> {
    // Create a new PythonSpy object with the default config options
    let config = py_spy::Config::default();
    let mut process = py_spy::PythonSpy::new(pid, &config)?;
    println!(
        "interpreter_address={:x} threadstate_address={:x}",
        process.interpreter_address, process.threadstate_address
    );

    // get stack traces for each thread in the process
    let traces = process.get_stack_traces()?;

    // Print out the python stack for each thread
    for trace in traces {
        println!(
            "Thread thread_id={} thread_id(hex)={:#X} os_thread_id={:?}",
            trace.thread_id,
            trace.thread_id,
            trace.os_thread_id,
        );
        // for frame in &trace.frames {
        //     println!("\t {} ({}:{})", frame.name, frame.filename, frame.line);
        // }
    }
    Ok(())
}
