use eframe::egui;

struct Question {
    prompt: String,
    answers: Vec<String>,
}

struct MyApp {
    questions: Vec<Question>,
    current_question: usize,
    user_input: String,
    feedback: String,
    answered_correctly: bool,
}

// Provides state for app UI interface
impl Default for MyApp {
    fn default() -> Self {
        Self {
            questions: vec![
                Question {
                prompt: "What flag in Bettercap enables the HTTP Proxy module?".to_string(),
                answers: vec!["-X".to_string()],
            },
            Question {
                prompt: "Which Ettercap mode performs ARP poisoning by default? (Answer: unified, bridged, or routing)".to_string(),
                answers: vec!["unified".to_string(), "bridged".to_string(), "routing".to_string()],
            },
            ],
            current_question: 0,
            user_input: String::new(),
            feedback: String::new(),
            answered_correctly: false,
        }
    }
}

// Implmentation for GUI logic & behavior
impl eframe::App for MyApp {
    // Calls to render and update UI box
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_question >= self.questions.len() {
                ui.heading("Workshop complete!");
                return;
            }

            // Get current question
            let current = &self.questions[self.current_question];

            ui.heading(format!(
                "Question {}/{}",
                self.current_question + 1,
                self.questions.len()
            ));
            ui.label(&current.prompt);

            // Get user input
            ui.text_edit_singleline(&mut self.user_input);

            // Check answer logic
            if ui.button("Submit").clicked() {
                let input = self
                    .user_input
                    .trim()
                    .to_lowercase()
                    .replace([' ', '-', '_'], "");

                let matched = current.answers.iter().any(|ans| {
                    let ans = ans.trim().to_lowercase().replace([' ', '-', '_'], "");

                    // Exact match
                    if input == ans {
                        return true;
                    } else {
                        return false;
                    }
                });

                if matched {
                    self.feedback = "✅ Correct!".to_string();
                    self.answered_correctly = true;
                } else {
                    self.feedback = "❌ Incorrect, try again.".to_string();
                    self.answered_correctly = false;
                }
            }

            // Display if answer is right or not
            ui.label(&self.feedback);

            // If answer was right, let the user move on to the next question
            if self.answered_correctly {
                // draw answer button
                if ui.button("Next").clicked() {
                    self.current_question += 1;
                    self.user_input.clear();
                    self.feedback.clear();
                    self.answered_correctly = false;
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default(); // default options

    // Start app
    eframe::run_native(
        "Ettercap Workshop",
        options,
        Box::new(|_cc| {
            // Initialzie with default state
            Box::new(MyApp {
                questions: MyApp::default().questions,
                current_question: 0,
                user_input: String::new(),
                feedback: String::new(),
                answered_correctly: false,
            })
        }),
    )
}
