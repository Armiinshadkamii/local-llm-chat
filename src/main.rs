mod model;

use eframe::egui;
use model::ModelManagement;

struct ChatApp {
    model_manager: Option<ModelManagement>,
    input: String,
    messages: Vec<(String, String)>, // role content
    model_path: String,
    is_loading: bool,
}

impl Default for ChatApp {
    fn default() -> Self {
        Self {
            model_manager: None,
            input: String::new(),
            messages: Vec::new(),
            model_path: String::from(r"C:\Users\PARSNOVIN\.cache\huggingface\hub\models--Qwen--Qwen2.5-7B-Instruct-GGUF\snapshots\bb5d59e06d9551d752d08b292a50eb208b07ab1f\qwen2.5-7b-instruct-q4_k_m-00001-of-00002.gguf"),
            is_loading: false
        }
    }
}

impl eframe::App for ChatApp {

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui: &mut egui::Ui| {
            ui.heading("local llm chat");
            ui.add_space(10.0);

            // --- Section 1: model loading ---
            if self.model_manager.is_none() {
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label("Model Path:");
                    ui.text_edit_singleline(&mut self.model_path);
                });
                ui.add_space(5.0);
                if ui.button("Load Model").clicked() {
                    self.is_loading = true;
                    // Force the UI to redraw so the "Loading..." text appears
                    ui.ctx().request_repaint();
                    match ModelManagement::load(&self.model_path) {
                        Ok(manager) => {
                            self.model_manager = Some(manager);
                            self.messages.push(("System".to_string(), "Model loaded successfully! Ready to chat.".to_string()));
                        }
                        Err(e) => {
                            self.messages.push(("Error".to_string(), format!("Failed to load model: {}", e)));
                            self.is_loading = false;
                        }
                    }

                    if self.is_loading {
                        ui.colored_label(egui::Color32::YELLOW, "Loading model into memory... (This may take 10-30 seconds)");
                    }
                }

            }
            else {
                // --- SECTION 2: Chat inference ---
                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui: &mut egui::Ui| {
                        for (role, content) in &self.messages {
                            let color = match role.as_str() {
                                "You" => egui::Color32::LIGHT_BLUE,
                                "Assistant" => egui::Color32::LIGHT_GREEN,
                                "Error" => egui::Color32::RED,
                                _ => egui::Color32::GRAY,
                            };
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{}:", role)).color(color).strong());
                                ui.label(content);
                            });
                            ui.add_space(5.0);
                        }
                    });

                    ui.separator();
                    ui.add_space(10.0);

                    // Input area
                    ui.horizontal(|ui: &mut egui::Ui|{
                        let input_field = ui.text_edit_singleline(&mut self.input);

                        let send_clicked = ui.button("Send").clicked();
                        let enter_pressed = input_field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                        if (send_clicked || enter_pressed) && !self.input.trim().is_empty() {
                        
                            let user_msg = self.input.clone();
                            self.messages.push(("You".to_string(), user_msg.clone()));

                            // Format prompt specifically for Qwen 2.5 chat template
                            let prompt = format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", user_msg);

                            if let Some(manager) = &mut self.model_manager {
                                match manager.generate(&prompt, 256) {
                                    Ok(response) => {
                                        self.messages.push(("Assistant".to_string(), response));
                                    }
                                    Err(e) => {
                                        self.messages.push(("Error".to_string(), format!("Generation failed: {}", e)));
                                    }
                                }
                            }

                            self.input.clear();
                            ui.ctx().request_repaint();
                        }
                    });
                
            }
        });
    }
    
}
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "LLM Local Chat",
        options,
        Box::new(|_cc| Ok(Box::<ChatApp>::default())),
    )?;
    Ok(())
}
