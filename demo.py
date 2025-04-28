import os
import threading
import time

K = 3


def thread_function(thread_index):
    thread_ident = threading.get_ident()
    thread_native_id = threading.get_native_id()

    for i in range(1000000000000):
        if i % 5000000 == 0:
            print(f"thread[{thread_index=}, {thread_ident=}, {thread_native_id=}] has {i=}")


def main():
    print(f"python program start {os.getpid()=}")
    threads = []
    for i in range(K):
        t = threading.Thread(target=thread_function, args=(i,))
        t.daemon = True
        t.start()
        threads.append(t)

    list_all_threads()

    try:
        while True:
            time.sleep(10)
    except KeyboardInterrupt:
        print("\nMain thread received KeyboardInterrupt. Exiting...")


def list_all_threads():
    text = "[All threads]"
    for thread in threading.enumerate():
        text += f" [{thread.name=} {thread.ident=} {thread.native_id=}]"
    print(text)


if __name__ == "__main__":
    main()
