pub const MODEL_OPENAI: &str = "gpt-4o-mini"; // o1-mini, gpt-4o-mini
pub const MODEL_ANTHROPIC: &str = "claude-3-haiku-20240307";
pub const MODEL_COHERE: &str = "command-light";
pub const MODEL_GEMINI: &str = "gemini-2.0-flash";
pub const MODEL_GROQ: &str = "llama3-8b-8192";
pub const MODEL_OLLAMA: &str = "gemma:2b"; // sh: `ollama pull gemma:2b`
pub const MODEL_XAI: &str = "grok-beta";
pub const MODEL_DEEPSEEK: &str = "deepseek-chat";

// NOTE: These are the default environment keys for each AI Adapter Type.
//       They can be customized; see `examples/c02-auth.rs`
pub const MODEL_AND_KEY_ENV_NAME_LIST: &[(&str, &str)] = &[
	// -- De/activate models/providers
	(MODEL_OPENAI, "OPENAI_API_KEY"),
	(MODEL_ANTHROPIC, "ANTHROPIC_API_KEY"),
	(MODEL_COHERE, "COHERE_API_KEY"),
	(MODEL_GEMINI, "GEMINI_API_KEY"),
	(MODEL_GROQ, "GROQ_API_KEY"),
	(MODEL_XAI, "XAI_API_KEY"),
	(MODEL_DEEPSEEK, "DEEPSEEK_API_KEY"),
	(MODEL_OLLAMA, ""),
];
