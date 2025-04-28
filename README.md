# py_gil_spy: Periodically (e.g. 1ms or shorter) dump which thread is holding the GIL lock in Python.

WARNING: This is just a very quick hacky ugly proof-of-concept script that I needed for a one-time quick check. If anyone wants to use it seriously, I am happy to spend a bit of time to make the code usable.

Below is a quick dump about how I use it.

## Example usage

SGLang + Mooncake + DeepEP has a bit of trouble and it looks like GIL issue. Then we can do the following:

```shell
cargo run --release 12345
```

where 12345 is pid of the python program. It will sample per 1ms and output a CSV file telling you the thread id that holds GIL at each 1ms sampling time.

At the same time, you may want to do profiling as usual, e.g.

```shell
curl -X POST "http://10.10.37.16:30303/start_profile"
# wait for a bit of time
curl -X POST "http://10.10.37.16:30303/stop_profile"
```

If you want to know which Python thread id corresponds to which OS thread id, you can use the branch https://github.com/fzyzcjy/sglang/tree/feat/modified_chwan_full-ep-pd and look at logs. Or just mimic the `list_all_threads` in demo.py. (There is better mechanism without touching python code but here again I am just hacking.)

If you want to know the absolute timestamp in pytorch profile file, there is a field `baseTimeNanoseconds` in the profile `.json` file, and the real timestamp of each perfetto event is the base time plus the shown time in perfetto.

Everything is hardwired in the Rust code (e.g. output path, supported Python version, etc). As mentioned above, please ping me if you want to use it other than SGLang debugging in 0428.

To make it compile, you need to fork py-spy and change the following line

```rust
// lib.rs
// NOTE MODIFIED add a `pub`
pub mod python_bindings;
```
