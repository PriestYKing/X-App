// use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use tokio::time::Duration;
// use uuid::Uuid;

// #[derive(Serialize, Deserialize)]
// pub struct AiRequest {
//     pub prompt: String,
//     pub context: Option<String>,
//     pub model: Option<String>,
//     pub max_tokens: Option<u32>,
//     pub temperature: Option<f32>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AiResponse {
//     pub id: Uuid,
//     pub response: String,
//     pub model_used: String,
//     pub tokens_used: u32,
//     pub processing_time_ms: u64,
// }

// #[derive(Clone)]
// pub struct AiService {
//     client: Client,
//     api_key: String,
//     base_url: String,
// }

// impl AiService {
//     pub fn new(api_key: String, base_url: Option<String>) -> Self {
//         Self {
//             client: Client::new(),
//             api_key,
//             base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
//         }
//     }
//     pub async fn generate_response(
//         &self,
//         request: AiRequest,
//     ) -> Result<AiResponse, Box<dyn std::error::Error>> {
//         let start_time = std::time::Instant::now();

//         // Extract the model value once to avoid moving it multiple times
//         let model_name = request.model.unwrap_or_else(|| "gpt-3.5-turbo".to_string());

//         // For OpenAI API
//         let openai_request = serde_json::json!({
//             "model": model_name,
//             "messages": [
//                 {
//                     "role": "system",
//                     "content": "You are a helpful AI assistant integrated into a social media platform similar to Twitter/X."
//                 },
//                 {
//                     "role": "user",
//                     "content": format!("{}\n\nContext: {}", request.prompt, request.context.unwrap_or_default())
//                 }
//             ],
//             "max_tokens": request.max_tokens.unwrap_or(150),
//             "temperature": request.temperature.unwrap_or(0.7)
//         });

//         let response = self
//             .client
//             .post(&format!("{}/chat/completions", self.base_url))
//             .header("Authorization", format!("Bearer {}", self.api_key))
//             .header("Content-Type", "application/json")
//             .json(&openai_request)
//             .send()
//             .await?;

//         let response_json: serde_json::Value = response.json().await?;
//         let ai_response = response_json["choices"][0]["message"]["content"]
//             .as_str()
//             .unwrap_or("Sorry, I couldn't generate a response.")
//             .to_string();

//         let processing_time = start_time.elapsed().as_millis() as u64;

//         Ok(AiResponse {
//             id: Uuid::new_v4(),
//             response: ai_response,
//             model_used: model_name, // Use the extracted value instead of moving again
//             tokens_used: response_json["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
//             processing_time_ms: processing_time,
//         })
//     }

//     pub async fn analyze_sentiment(
//         &self,
//         text: &str,
//     ) -> Result<String, Box<dyn std::error::Error>> {
//         let request = AiRequest {
//             prompt: format!("Analyze the sentiment of this text and return only one word (positive, negative, or neutral): '{}'", text),
//             context: None,
//             model: Some("gpt-3.5-turbo".to_string()),
//             max_tokens: Some(10),
//             temperature: Some(0.1),
//         };

//         let response = self.generate_response(request).await?;
//         Ok(response.response.trim().to_lowercase())
//     }

//     pub async fn generate_hashtags(
//         &self,
//         content: &str,
//     ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//         let request = AiRequest {
//             prompt: format!("Generate 3-5 relevant hashtags for this social media post. Return only the hashtags separated by spaces, without explanations: '{}'", content),
//             context: None,
//             model: Some("gpt-3.5-turbo".to_string()),
//             max_tokens: Some(50),
//             temperature: Some(0.5),
//         };

//         let response = self.generate_response(request).await?;
//         let hashtags: Vec<String> = response
//             .response
//             .split_whitespace()
//             .filter(|tag| tag.starts_with('#'))
//             .map(|tag| tag.to_string())
//             .collect();

//         Ok(hashtags)
//     }

//     pub async fn moderate_content(
//         &self,
//         content: &str,
//     ) -> Result<bool, Box<dyn std::error::Error>> {
//         let moderation_request = serde_json::json!({
//             "input": content
//         });

//         let response = self
//             .client
//             .post(&format!("{}/moderations", self.base_url))
//             .header("Authorization", format!("Bearer {}", self.api_key))
//             .header("Content-Type", "application/json")
//             .json(&moderation_request)
//             .send()
//             .await?;

//         let response_json: serde_json::Value = response.json().await?;
//         let is_flagged = response_json["results"][0]["flagged"]
//             .as_bool()
//             .unwrap_or(false);

//         Ok(!is_flagged) // Return true if content is safe (not flagged)
//     }

//     pub async fn summarize_thread(
//         &self,
//         posts: Vec<String>,
//     ) -> Result<String, Box<dyn std::error::Error>> {
//         let combined_text = posts.join("\n\n");
//         let request = AiRequest {
//             prompt: format!(
//                 "Summarize this Twitter thread in 2-3 sentences:\n\n{}",
//                 combined_text
//             ),
//             context: None,
//             model: Some("gpt-3.5-turbo".to_string()),
//             max_tokens: Some(100),
//             temperature: Some(0.3),
//         };

//         let response = self.generate_response(request).await?;
//         Ok(response.response)
//     }

//     pub async fn generate_reply_suggestions(
//         &self,
//         original_post: &str,
//     ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//         let request = AiRequest {
//             prompt: format!("Generate 3 different reply suggestions for this social media post. Keep them short and engaging. Return only the replies, one per line:\n\n'{}'", original_post),
//             context: None,
//             model: Some("gpt-3.5-turbo".to_string()),
//             max_tokens: Some(100),
//             temperature: Some(0.8),
//         };

//         let response = self.generate_response(request).await?;
//         let suggestions: Vec<String> = response
//             .response
//             .lines()
//             .map(|line| line.trim().to_string())
//             .filter(|line| !line.is_empty())
//             .take(3)
//             .collect();

//         Ok(suggestions)
//     }

//     pub async fn detect_language(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
//         let request = AiRequest {
//             prompt: format!("Detect the language of this text and return only the language name in English: '{}'", text),
//             context: None,
//             model: Some("gpt-3.5-turbo".to_string()),
//             max_tokens: Some(10),
//             temperature: Some(0.1),
//         };

//         let response = self.generate_response(request).await?;
//         Ok(response.response.trim().to_string())
//     }

//     pub async fn generate_content_warning(
//         &self,
//         content: &str,
//     ) -> Result<Option<String>, Box<dyn std::error::Error>> {
//         let request = AiRequest {
//             prompt: format!("Analyze this social media post and determine if it needs a content warning. If yes, return a brief warning (e.g., 'Contains violence', 'Sensitive political content'). If no warning needed, return 'NONE':\n\n'{}'", content),
//             context: None,
//             model: Some("gpt-3.5-turbo".to_string()),
//             max_tokens: Some(50),
//             temperature: Some(0.2),
//         };

//         let response = self.generate_response(request).await?;
//         let warning = response.response.trim();

//         if warning.to_uppercase() == "NONE" {
//             Ok(None)
//         } else {
//             Ok(Some(warning.to_string()))
//         }
//     }
// }

// // Local LLM Service using Candle
// #[cfg(feature = "local-llm")]
// pub struct LocalLLMService {
//     model: Option<Box<dyn LocalModel + Send + Sync>>,
//     tokenizer: Option<tokenizers::Tokenizer>,
//     device: candle_core::Device,
// }

// #[cfg(feature = "local-llm")]
// pub trait LocalModel {
//     fn forward(&mut self, tokens: &candle_core::Tensor) -> anyhow::Result<candle_core::Tensor>;
//     fn model_type(&self) -> &str;
// }

// #[cfg(feature = "local-llm")]
// pub struct LlamaModel {
//     model: candle_transformers::models::llama::Llama,
//     config: candle_transformers::models::llama::Config,
// }

// #[cfg(feature = "local-llm")]
// impl LocalModel for LlamaModel {
//     fn forward(&mut self, tokens: &candle_core::Tensor) -> anyhow::Result<candle_core::Tensor> {
//         self.model.forward(tokens, 0)
//     }

//     fn model_type(&self) -> &str {
//         "Llama"
//     }
// }

// #[cfg(feature = "local-llm")]
// impl LocalLLMService {
//     pub async fn new(
//         model_path: &str,
//         tokenizer_path: &str,
//     ) -> Result<Self, Box<dyn std::error::Error>> {
//         use candle_core::Device;
//         use std::path::Path;

//         // Initialize device (prefer CUDA if available, fallback to CPU)
//         let device = if candle_core::utils::cuda_is_available() {
//             Device::new_cuda(0)?
//         } else {
//             Device::Cpu
//         };

//         log::info!("Using device: {:?}", device);

//         // Load tokenizer
//         let tokenizer = tokenizers::Tokenizer::from_file(tokenizer_path)
//             .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

//         // Load model (this is a simplified example - you'll need to adapt based on your model format)
//         let model = Self::load_llama_model(model_path, &device).await?;

//         Ok(Self {
//             model: Some(Box::new(model)),
//             tokenizer: Some(tokenizer),
//             device,
//         })
//     }

//     async fn load_llama_model(
//         model_path: &str,
//         device: &candle_core::Device,
//     ) -> anyhow::Result<LlamaModel> {
//         use candle_nn::VarBuilder;
//         use candle_transformers::models::llama::{Config, Llama};
//         use std::path::Path;

//         // This is a simplified example - you'll need to implement proper model loading
//         // based on your specific model format (safetensors, GGUF, etc.)

//         // Load config (you might need to adjust this based on your model)
//         let config = Config::config_7b_v1(); // Default 7B config, adjust as needed

//         // Load model weights
//         let model_path = Path::new(model_path);

//         // For safetensors format
//         if model_path.extension().and_then(|s| s.to_str()) == Some("safetensors") {
//             let tensors = candle_core::safetensors::load(model_path, device)?;
//             let var_builder = VarBuilder::from_tensors(tensors, candle_core::DType::F32, device);
//             let model = Llama::load(&var_builder, &config)?;

//             Ok(LlamaModel { model, config })
//         } else {
//             return Err(anyhow::anyhow!(
//                 "Unsupported model format. Please use safetensors format."
//             ));
//         }
//     }

//     pub async fn generate_local_response(
//         &mut self,
//         prompt: &str,
//         max_tokens: Option<usize>,
//         temperature: Option<f32>,
//     ) -> Result<String, Box<dyn std::error::Error>> {
//         let max_tokens = max_tokens.unwrap_or(150);
//         let temperature = temperature.unwrap_or(0.8);

//         if let (Some(model), Some(tokenizer)) = (&mut self.model, &self.tokenizer) {
//             // Encode the prompt
//             let encoding = tokenizer
//                 .encode(prompt, false)
//                 .map_err(|e| format!("Tokenization failed: {}", e))?;

//             let tokens = encoding.get_ids();
//             let input_tokens = candle_core::Tensor::new(tokens, &self.device)?;

//             // Generate response
//             let mut generated_tokens = Vec::new();
//             let mut current_tokens = input_tokens;

//             for _ in 0..max_tokens {
//                 // Forward pass
//                 let logits = model.forward(&current_tokens)?;

//                 // Apply temperature and sample
//                 let next_token = self.sample_token(&logits, temperature)?;

//                 // Check for end-of-sequence token
//                 if next_token == tokenizer.token_to_id("<|endoftext|>").unwrap_or(0) {
//                     break;
//                 }

//                 generated_tokens.push(next_token);

//                 // Update current tokens for next iteration
//                 let new_tokens = [tokens, &generated_tokens].concat();
//                 current_tokens = candle_core::Tensor::new(&new_tokens, &self.device)?;
//             }

//             // Decode the generated tokens
//             let response = tokenizer
//                 .decode(&generated_tokens, true)
//                 .map_err(|e| format!("Decoding failed: {}", e))?;

//             Ok(response)
//         } else {
//             Err("Model not initialized".into())
//         }
//     }

//     fn sample_token(&self, logits: &candle_core::Tensor, temperature: f32) -> anyhow::Result<u32> {
//         use candle_core::Tensor;

//         // Apply temperature scaling
//         let scaled_logits = logits.broadcast_div(&Tensor::new(&[temperature], logits.device())?)?;

//         // Apply softmax
//         let probabilities = candle_nn::ops::softmax(&scaled_logits, candle_core::D::Minus1)?;

//         // Sample from the distribution (simplified - you might want to implement top-k or nucleus sampling)
//         let probabilities_vec = probabilities.to_vec1::<f32>()?;
//         let token_id = self.sample_from_distribution(&probabilities_vec)?;

//         Ok(token_id as u32)
//     }

//     fn sample_from_distribution(&self, probabilities: &[f32]) -> anyhow::Result<usize> {
//         use rand::Rng;

//         let mut rng = rand::thread_rng();
//         let random_value: f32 = rng.gen();

//         let mut cumulative_prob = 0.0;
//         for (i, &prob) in probabilities.iter().enumerate() {
//             cumulative_prob += prob;
//             if random_value <= cumulative_prob {
//                 return Ok(i);
//             }
//         }

//         // Fallback to last token
//         Ok(probabilities.len() - 1)
//     }

//     pub async fn analyze_sentiment_local(
//         &mut self,
//         text: &str,
//     ) -> Result<String, Box<dyn std::error::Error>> {
//         let prompt = format!("Analyze the sentiment of this text and return only one word (positive, negative, or neutral): '{}'", text);
//         let response = self
//             .generate_local_response(&prompt, Some(10), Some(0.1))
//             .await?;
//         Ok(response.trim().to_lowercase())
//     }

//     pub async fn generate_hashtags_local(
//         &mut self,
//         content: &str,
//     ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//         let prompt = format!("Generate 3-5 relevant hashtags for this social media post. Return only the hashtags separated by spaces: '{}'", content);
//         let response = self
//             .generate_local_response(&prompt, Some(50), Some(0.5))
//             .await?;

//         let hashtags: Vec<String> = response
//             .split_whitespace()
//             .filter(|tag| tag.starts_with('#'))
//             .map(|tag| tag.to_string())
//             .collect();

//         Ok(hashtags)
//     }
// }

// // Mock implementation when local-llm feature is disabled
// #[cfg(not(feature = "local-llm"))]
// pub struct LocalLLMService;

// #[cfg(not(feature = "local-llm"))]
// impl LocalLLMService {
//     pub async fn new(
//         _model_path: &str,
//         _tokenizer_path: &str,
//     ) -> Result<Self, Box<dyn std::error::Error>> {
//         log::warn!("Local LLM service not available. Compile with --features local-llm to enable.");
//         Ok(LocalLLMService)
//     }

//     pub async fn generate_local_response(
//         &mut self,
//         _prompt: &str,
//         _max_tokens: Option<usize>,
//         _temperature: Option<f32>,
//     ) -> Result<String, Box<dyn std::error::Error>> {
//         Err("Local LLM service not available. Use AiService for remote AI features.".into())
//     }

//     pub async fn analyze_sentiment_local(
//         &mut self,
//         _text: &str,
//     ) -> Result<String, Box<dyn std::error::Error>> {
//         Err("Local LLM service not available.".into())
//     }

//     pub async fn generate_hashtags_local(
//         &mut self,
//         _content: &str,
//     ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//         Err("Local LLM service not available.".into())
//     }
// }

// #[cfg(feature = "local-llm")]
// mod candle_utils {
//     use candle_core::{Device, Tensor};
//     use hf_hub::api::tokio::Api;
//     use std::path::PathBuf;

//     pub async fn download_model(model_id: &str, filename: &str) -> anyhow::Result<PathBuf> {
//         let api = Api::new()?;
//         let repo = api.model(model_id.to_string());
//         let model_path = repo.get(filename).await?;
//         Ok(model_path)
//     }

//     pub async fn download_tokenizer(model_id: &str) -> anyhow::Result<PathBuf> {
//         let api = Api::new()?;
//         let repo = api.model(model_id.to_string());
//         let tokenizer_path = repo.get("tokenizer.json").await?;
//         Ok(tokenizer_path)
//     }
// }
