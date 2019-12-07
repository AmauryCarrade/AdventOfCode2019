use crate::intcode::{Error, Program};
use crate::{first_answer, input, second_answer};

use itertools::Itertools;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc,
};
use std::thread;
use std::thread::JoinHandle;

/// Runs an amplifier. This should be executed in a thread.
/// Data should be received from `rx`, and output sent as they go to `tx`.
fn run_amplifier(source_code: Arc<String>, rx: Receiver<i64>, tx: Sender<i64>) {
    let mut program: Program = source_code.parse().unwrap();

    program.set_input(move |_| {
        rx.recv().map_err(|_| Error {
            message: "Cannot receive input",
        })
    });

    loop {
        match program.execute_until_next_output() {
            Ok(output) => {
                // We ignore transmissions error as programs may run an extra, harmless, step
                // and transmit to an already-closed thread. We don't care.
                let _ = tx.send(output);

                if !program.is_running() {
                    break;
                }
            }
            Err(e) => panic!(e),
        }
    }
}

pub fn run() {
    let source_code = Arc::new(input(7).first().expect("Empty source code").clone());

    first_answer(
        "Highest output signal",
        &(0..5)
            .permutations(5)
            .map(|phase_setting_sequence| {
                phase_setting_sequence
                    .into_iter()
                    .fold(0, |output_signal, setting| {
                        let mut program: Program = Arc::clone(&source_code).parse().unwrap();
                        program.set_input(move |n| match n {
                            0 => Ok(setting.clone() as i64),
                            1 => Ok(output_signal.clone() as i64),
                            _ => Err(Error {
                                message: "Too many inputs",
                            }),
                        });

                        match program.execute() {
                            Ok(output) => output[0],
                            Err(e) => panic!(e), // too lazy for proper error handling
                        }
                    })
            })
            .max()
            .unwrap(),
    );

    second_answer(
        "Highest output signal with feedback loop",
        &(5..10)
            .permutations(5)
            .map(|phase_setting_sequence| {
                let (tx_input, rx_a) = channel();
                let (tx_a, rx_b) = channel();
                let (tx_b, rx_c) = channel();
                let (tx_c, rx_d) = channel();
                let (tx_d, rx_e) = channel();
                let (tx_e, rx_output) = channel();

                tx_input
                    .send(phase_setting_sequence[0])
                    .expect("Unable to send phase setting to amplifier A");
                tx_a.send(phase_setting_sequence[1])
                    .expect("Unable to send phase setting to amplifier B");
                tx_b.send(phase_setting_sequence[2])
                    .expect("Unable to send phase setting to amplifier C");
                tx_c.send(phase_setting_sequence[3])
                    .expect("Unable to send phase setting to amplifier D");
                tx_d.send(phase_setting_sequence[4])
                    .expect("Unable to send phase setting to amplifier E");

                let amplifiers_threads: Vec<JoinHandle<_>> = vec![
                    (rx_a, tx_a),
                    (rx_b, tx_b),
                    (rx_c, tx_c),
                    (rx_d, tx_d),
                    (rx_e, tx_e),
                ]
                .into_iter()
                .map(|(rx, tx)| {
                    let atomic_source_code = Arc::clone(&source_code);
                    thread::spawn(move || run_amplifier(atomic_source_code, rx, tx))
                })
                .collect();

                tx_input
                    .send(0)
                    .expect("Unable to send startup signal to first amplifier");

                let mut output = 0;

                while let Ok(amplifier_output) = rx_output.recv() {
                    output = amplifier_output;

                    // Same as in `run_amplifier`: we don't care if the transmission fails.
                    let _ = tx_input.send(amplifier_output);
                }

                for thread in amplifiers_threads {
                    thread.join().expect("Unable to join ACS thread");
                }

                output
            })
            .max()
            .unwrap(),
    );
}
