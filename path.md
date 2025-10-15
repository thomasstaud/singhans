| Phase                              | Goal                                            | Key Crates        |
| ---------------------------------- | ----------------------------------------------- | ----------------- |
| **1. Audio basics**                | Output a sine wave, control frequency with code | `cpal`            |
| **2. MIDI control**                | Respond to a MIDI keyboard                      | `cpal`, `midir`   |
| **3. Polyphony**                   | Multiple voices, envelopes                      | `dasp`, `ringbuf` |
| **4. Filters**                     | Add tone shaping                                | `biquad`, `dasp`  |
| **5. Modulation**                  | LFOs, envelopes, routing                        | your own structs  |
| **6. GUI + preset system**         | Visual control, patch saving                    | `egui`, `serde`   |
| **7. JACK integration (optional)** | Linux ecosystem                                 | `jack`            |
