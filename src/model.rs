/*
This file is responsible for model management.
*/

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::sampling::LlamaSampler;

pub struct ModelManagement {
    _backend: LlamaBackend,
    model: LlamaModel,
}

impl ModelManagement {
    pub fn load(model_path:&str) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Loading model from {}", model_path);

        let backend:LlamaBackend = LlamaBackend::init()?;
        let params: LlamaModelParams = LlamaModelParams::default()
            .with_n_gpu_layers(35); // builders return a new instance - not in place.

        println!("Loading model with {} GPU layers...", params.n_gpu_layers());
        let model: LlamaModel = LlamaModel::load_from_file(&backend, model_path, &params)?;
        println!("model loaded successfully");

        Ok(Self { _backend: backend, model })
    }

    pub fn generate(&mut self, prompt: &str, max_tokens:i32) -> Result<String, Box<dyn std::error::Error>> {

        let ctx_params = llama_cpp_2::context::params::LlamaContextParams::default()
            .with_n_ctx(Some(std::num::NonZeroU32::new(2048).expect("2048 is non-zero")));

        let mut ctx = self.model
            .new_context(&self._backend, ctx_params)?;

        let tokens_list = self.model.vocab().tokenize(prompt.as_bytes(), true, true);
        let last_idx = tokens_list.len() as i32 -1;

        let mut batch: LlamaBatch<'_> = LlamaBatch::new(512, 1);

        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            let is_last: bool = i == last_idx;
            batch.add(token, i, &[0], is_last)?;
        }

        ctx.decode(&mut batch)?;
        let mut n_cur = batch.n_tokens();

        let mut output = String::new();
        let mut sampler = LlamaSampler::greedy();

        while n_cur <= max_tokens {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            
            if self.model.vocab().is_eog(token) {
                break;
            }

            // Convert token to text and add to our output string
            let piece = self.model.vocab().token_to_piece(token, true, None);
            // from_utf8_lossy safely handles any weird sub-word byte fragments
            output.push_str(&String::from_utf8_lossy(&piece));

            // Prepare the batch for the NEXT loop iteration (just this one new token)
            batch.clear();
            batch.add(token, n_cur, &[0], true)?;
            
            // Push the new token into the engine
            ctx.decode(&mut batch)?;
            n_cur += 1;
        }

        Ok(output)
    }
}