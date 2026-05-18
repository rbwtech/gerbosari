//! Admin image upload endpoint.
//!
//! `POST /api/admin/upload` accepts a `multipart/form-data` body with:
//!   - `file`:   the binary image (required, max 5 MiB)
//!   - `entity`: target subdirectory under `upload_dir` (optional;
//!               defaults to `gallery`; whitelisted to `gallery` | `berita`)
//!
//! Allowed MIME types: `image/jpeg`, `image/png`, `image/webp`. Anything else
//! is refused with 400 — this is the cheapest defence against arbitrary file
//! upload before we ever touch disk. The destination path is built from a
//! UUIDv4 + safe extension, so an attacker cannot influence the on-disk name
//! even if they manage to set `Content-Type`.

use std::path::PathBuf;

use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::presentation::middleware::jwt::AuthGuard;
use crate::presentation::state::AppState;

/// Hard cap on accepted file size. Larger requests are also blocked at the
/// router layer by `DefaultBodyLimit`, but we re-check after streaming so the
/// limit is enforced even if the router layer is ever swapped out.
const MAX_FILE_BYTES: usize = 5 * 1024 * 1024;

/// Whitelisted destination subdirectories. Refuses path traversal by design —
/// the value is matched against this set, not joined raw.
const ALLOWED_ENTITIES: &[&str] = &["gallery", "berita"];

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    /// Public URL the SPA should store in the entity's `file_path` /
    /// `gambar_url` column. Always begins with `/images/<entity>/`.
    pub url: String,
    /// Bytes written to disk. Useful for surface validation in the UI.
    pub size: u64,
    /// On-disk filename (UUIDv4 + extension). Exposed so the SPA can show it
    /// in a preview line; not the public URL.
    pub filename: String,
}

/// Multipart-parsed view of the incoming form. Built incrementally inside
/// `admin_upload` so the unwrapping of `Option<…>` lives in one place.
struct ParsedUpload {
    bytes: Vec<u8>,
    entity: String,
    extension: &'static str,
}

pub async fn admin_upload(
    _auth: AuthGuard,
    State(state): State<AppState>,
    multipart: Multipart,
) -> Result<(StatusCode, Json<UploadResponse>), AppError> {
    let parsed = parse_multipart(multipart).await?;

    let filename = format!("{}.{}", Uuid::new_v4().simple(), parsed.extension);
    let dir: PathBuf = state.upload_dir.join(&parsed.entity);

    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("upload mkdir failed: {e}")))?;

    let dest = dir.join(&filename);
    tokio::fs::write(&dest, &parsed.bytes)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("upload write failed: {e}")))?;

    let url = format!("/images/{}/{}", parsed.entity, filename);
    let size = parsed.bytes.len() as u64;

    tracing::info!(
        target: "gerbosari::upload",
        entity = %parsed.entity,
        filename = %filename,
        size = size,
        "admin upload"
    );

    Ok((
        StatusCode::CREATED,
        Json(UploadResponse {
            url,
            size,
            filename,
        }),
    ))
}

/// Drains the multipart stream and validates fields. Returns `BadRequest`
/// on any client-fixable issue (missing file, bad type, oversize) so the
/// SPA can surface the message verbatim.
async fn parse_multipart(mut multipart: Multipart) -> Result<ParsedUpload, AppError> {
    let mut bytes: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;
    let mut entity: String = "gallery".to_string();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("invalid multipart: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "entity" => {
                let raw = field
                    .text()
                    .await
                    .map_err(|e| AppError::BadRequest(format!("invalid entity field: {e}")))?;
                let trimmed = raw.trim();
                if !ALLOWED_ENTITIES.contains(&trimmed) {
                    return Err(AppError::BadRequest(format!(
                        "entity must be one of: {}",
                        ALLOWED_ENTITIES.join(", ")
                    )));
                }
                entity = trimmed.to_string();
            }
            "file" => {
                // Capture content_type BEFORE consuming the field with .bytes().
                content_type = field.content_type().map(|s| s.to_string());
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::BadRequest(format!("failed to read file: {e}")))?;
                if data.len() > MAX_FILE_BYTES {
                    return Err(AppError::BadRequest(format!(
                        "file too large; max {} MiB",
                        MAX_FILE_BYTES / (1024 * 1024)
                    )));
                }
                if data.is_empty() {
                    return Err(AppError::BadRequest("file is empty".into()));
                }
                bytes = Some(data.to_vec());
            }
            _ => {
                // Drain unknown fields so the stream advances; ignore the value.
                let _ = field.bytes().await;
            }
        }
    }

    let bytes = bytes.ok_or_else(|| AppError::BadRequest("missing 'file' field".into()))?;

    let extension = match content_type.as_deref() {
        Some("image/jpeg") | Some("image/jpg") => "jpg",
        Some("image/png") => "png",
        Some("image/webp") => "webp",
        Some(other) => {
            return Err(AppError::BadRequest(format!(
                "unsupported file type: {other}. Allowed: image/jpeg, image/png, image/webp"
            )))
        }
        None => {
            return Err(AppError::BadRequest(
                "missing Content-Type on 'file' field".into(),
            ))
        }
    };

    Ok(ParsedUpload {
        bytes,
        entity,
        extension,
    })
}
