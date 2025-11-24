use std::{env, fs, path::Path};

use zed::LanguageServerId;
use zed_extension_api::{
    self as zed,
    lsp::{Completion, CompletionKind},
    serde_json::{json},
    settings::LspSettings,
    CodeLabelSpan, Result,
};

struct As3Extension;

impl zed::Extension for As3Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let Some(java_path) = worktree.which("java") else {
            return Err("Java is not installed, please install Java >= 11".to_string());
        };

        let initialization_options = LspSettings::for_worktree("actionscript", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options);

        zed::set_language_server_installation_status(
            _language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "BowlerHatLLC/vscode-as3mxml",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let as3mxml_dir = format!("vscode-as3mxml-{}", release.version);

        if !fs::metadata(&as3mxml_dir).map_or(false, |stat| stat.is_dir()) {
            zed::set_language_server_installation_status(
                _language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(
                &release.assets[0].download_url,
                &as3mxml_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;
        }
        let sdk_path = initialization_options.as_ref().and_then(|s| {
            s.get("sdk_path")
                .and_then(|v| v.as_str())
                .map(|v| v.to_string())
        });

        if sdk_path.is_none() {
            return Err("SDK path not configured in settings, see https://github.com/pngdrift/zed-actionscript/".to_string());
        }

        let sdk_path = sdk_path.unwrap();

        let frameworks_path = Path::new(&sdk_path).join("frameworks");

        /* if !frameworks_path.exists() {
            return Err("SDK frameworks path not exists".to_string());
        } */

        let path_delimiter = match zed::current_platform().0 {
            zed::Os::Windows => ";",
            _ => ":",
        };

        let absolute_as3mxml_path =
            Path::new(env::current_dir().unwrap().to_str().unwrap()).join(as3mxml_dir);

        Ok(zed::Command {
            command: java_path.to_string(),
            args: vec![
                format!("-Droyalelib={}", frameworks_path.display().to_string()),
                "-Dfile.encoding=UTF8".to_string(),
                "-cp".to_string(),
                format!(
                    "{0}/extension/bundled-compiler/*{1}{0}/extension/bin/*",
                    absolute_as3mxml_path.to_string_lossy(),
                    path_delimiter
                ),
                "com.as3mxml.vscode.Main".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let initialization_options = LspSettings::for_worktree("actionscript", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        let settings = json!({
            "as3mxml": initialization_options
        });
        Ok(Some(settings))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: Completion,
    ) -> Option<zed::CodeLabel> {
        let highlight_name = match completion.kind? {
            CompletionKind::Class | CompletionKind::Interface => "type",
            CompletionKind::Constructor => "type",
            CompletionKind::Constant => "constant",
            CompletionKind::Function | CompletionKind::Method => "function",
            CompletionKind::Property | CompletionKind::Field => "property",
            CompletionKind::Variable => "variable",
            CompletionKind::Keyword => "keyword",
            CompletionKind::Value => "value",
            _ => return None,
        };

        let len = completion.label.len();
        let name_span: CodeLabelSpan =
            CodeLabelSpan::literal(completion.label, Some(highlight_name.to_string()));

        Some(zed::CodeLabel {
            code: Default::default(),
            spans: if let Some(detail) = completion.detail {
                vec![
                    name_span,
                    CodeLabelSpan::literal(" ", None),
                    CodeLabelSpan::literal(detail, None),
                ]
            } else {
                vec![name_span]
            },
            filter_range: (0..len).into(),
        })
    }
}

zed::register_extension!(As3Extension);
