use crate::retrieval::context::Context;

/// Prompt passed to the reasoning engine.
///
/// This is intentionally model-agnostic.
/// It can later be serialized into:
///
/// - TinyLM prompt
/// - GGUF models
/// - ONNX models
/// - Future local models
#[derive(Debug, Clone)]
pub struct Prompt {
    system: String,
    user: String,
    context: String,
}

impl Prompt {
    pub fn system(&self) -> &str {
        &self.system
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn context(&self) -> &str {
        &self.context
    }
}

pub struct PromptBuilder {
    system_prompt: String,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            system_prompt: String::from(
                "You are Crying Obsidian, a local knowledge intelligence assistant. \
Answer using only the supplied context. \
If the answer is not present, say that it could not be found.",
            ),
        }
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    pub fn build(&self, question: impl AsRef<str>, context: &Context) -> Prompt {
        let mut ctx = String::new();

        for document in context.documents() {
            ctx.push_str("# ");
            ctx.push_str(document.title());
            ctx.push('\n');

            ctx.push_str(document.text());

            ctx.push_str("\n\n");
        }

        Prompt {
            system: self.system_prompt.clone(),
            user: question.as_ref().to_owned(),
            context: ctx,
        }
    }
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_system_prompt() {
        let builder = PromptBuilder::new().with_system_prompt("Custom");

        assert_eq!(builder.system_prompt, "Custom");
    }
}
