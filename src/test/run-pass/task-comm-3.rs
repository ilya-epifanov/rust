use std;
import task;
import comm;
import comm::chan;
import comm::send;
import comm::recv;

fn main() { #debug("===== WITHOUT THREADS ====="); test00(); }

fn test00_start(ch: chan<int>, message: int, count: int) {
    #debug("Starting test00_start");
    let i: int = 0;
    while i < count {
        #debug("Sending Message");
        send(ch, message + 0);
        i = i + 1;
    }
    #debug("Ending test00_start");
}

fn test00() {
    let number_of_tasks: int = 16;
    let number_of_messages: int = 4;

    #debug("Creating tasks");

    let po = comm::port();
    let ch = chan(po);

    let i: int = 0;

    // Create and spawn tasks...
    let results = [];
    while i < number_of_tasks {
        let builder = task::mk_task_builder();
        results += [task::future_result(builder)];
        task::run(builder) {||
            test00_start(ch, i, number_of_messages)
        }
        i = i + 1;
    }

    // Read from spawned tasks...
    let sum = 0;
    for r in results {
        i = 0;
        while i < number_of_messages {
            let value = recv(po);
            sum += value;
            i = i + 1;
        }
    }

    // Join spawned tasks...
    for r in results { future::get(r); }

    #debug("Completed: Final number is: ");
    log(error, sum);
    // assert (sum == (((number_of_tasks * (number_of_tasks - 1)) / 2) *
    //       number_of_messages));
    assert (sum == 480);
}
