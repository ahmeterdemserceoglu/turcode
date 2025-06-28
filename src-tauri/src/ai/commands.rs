use tauri::command;
use tauri::State;
use crate::ai::{AIService, CodeCompletionResponse, CodeAnalysisResponse};

#[command]
pub async fn get_code_completion(
    ai_service: State<'_, AIService>,
    context: String,
    prompt: String,
) -> Result<CodeCompletionResponse, String> {
    let client = ai_service.get_client().await?;
    client.get_code_completion(&context, &prompt).await
}

#[command]
pub async fn analyze_code(
    ai_service: State<'_, AIService>,
    code: String,
    language: String,
) -> Result<CodeAnalysisResponse, String> {
    let client = ai_service.get_client().await?;
    client.analyze_code(&code, &language).await
} 