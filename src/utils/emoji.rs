pub(crate) fn get_file_emoji(extension: &str) -> &'static str {
    match extension.to_lowercase().as_str() {
        // Images
        "png" | "gif" | "bmp" | "svg" | "ico" | "tiff" | "tif" => "🖼️",
        "webp" | "jpg" | "jpeg" | "raw" | "cr2" | "nef" | "arw" | "heic" => "📷",

        // Documents
        "txt" => "📄",
        "pdf" => "📕",
        "doc" | "docx" => "📘",
        "xls" | "xlsx" | "csv" => "📊",
        "ppt" | "pptx" => "📈",
        "rtf" => "📝",

        // Code
        "rs" => "🦀",
        "py" => "🐍",
        "js" | "ts" | "jsx" | "tsx" => "📜",
        "html" | "htm" => "🌐",
        "css" | "scss" | "sass" => "🎨",
        "java" => "☕",
        "cpp" | "c" | "cc" | "cxx" | "h" | "hpp" => "⚙️",
        "cs" => "🔷",
        "php" => "🐘",
        "rb" => "💎",
        "go" => "🐹",
        "swift" => "🦉",
        "r" => "📊",
        "sh" | "bash" | "zsh" | "fish" => "🐚",

        // Archives
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => "🗃️",
        // Audio
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" => "🎵",
        // Video
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "🎬",
        // Executables
        "exe" | "msi" | "app" | "deb" | "rpm" | "dmg" => "⚡",
        // Configuration
        "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" => "⚙️",
        // Fonts
        "ttf" | "otf" | "woff" | "woff2" | "eot" => "🔤",
        // Databases
        "db" | "sqlite" | "sqlite3" | "mdb" => "🗄️",
        // Logs
        "log" => "📋",
        // License/Legal
        "license" | "licence" => "📜",
        // Markdown
        "md" | "markdown" => "📖",
        // Unknown
        _ => "❓",
    }
}
