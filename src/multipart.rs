//! multipart/form-data gövdesi (içe aktarma ve CSV yükleme uçları).

use std::fmt;

/// Multipart dosya alanı. İçerik bellekte tutulur ki yeniden denemeler aynı gövdeyi göndersin.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileUpload {
    /// Dosya adı (boşsa `upload`).
    pub filename: String,
    /// İçerik tipi (boşsa `application/octet-stream`).
    pub content_type: String,
    /// Dosya içeriği.
    pub content: Vec<u8>,
}

impl FileUpload {
    /// Bellekteki içerikten dosya alanı.
    pub fn new(filename: impl Into<String>, content_type: impl Into<String>, content: impl Into<Vec<u8>>) -> Self {
        Self { filename: filename.into(), content_type: content_type.into(), content: content.into() }
    }
}

enum Part {
    Field(String, String),
    File(String, FileUpload),
}

pub(crate) struct Multipart {
    parts: Vec<Part>,
}

impl Multipart {
    pub(crate) fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub(crate) fn field<V: fmt::Display + ?Sized>(&mut self, name: &str, value: &V) {
        self.parts.push(Part::Field(name.to_string(), value.to_string()));
    }

    pub(crate) fn file(&mut self, name: &str, file: &FileUpload) {
        self.parts.push(Part::File(name.to_string(), file.clone()));
    }

    /// Gövde baytları ve `Content-Type` başlığı.
    pub(crate) fn encode(&self) -> (Vec<u8>, String) {
        let boundary = format!("pushizo-{}", random_hex(16));
        let mut out: Vec<u8> = Vec::new();
        for part in &self.parts {
            out.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
            match part {
                Part::Field(name, value) => {
                    out.extend_from_slice(
                        format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", escape_quotes(name)).as_bytes(),
                    );
                    out.extend_from_slice(value.as_bytes());
                }
                Part::File(name, file) => {
                    let filename = if file.filename.is_empty() { "upload" } else { file.filename.as_str() };
                    let content_type = if file.content_type.is_empty() {
                        "application/octet-stream"
                    } else {
                        file.content_type.as_str()
                    };
                    out.extend_from_slice(
                        format!(
                            "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
                            escape_quotes(name),
                            escape_quotes(filename),
                            content_type
                        )
                        .as_bytes(),
                    );
                    out.extend_from_slice(&file.content);
                }
            }
            out.extend_from_slice(b"\r\n");
        }
        out.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
        (out, format!("multipart/form-data; boundary={boundary}"))
    }
}

fn escape_quotes(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace(['\r', '\n'], " ")
}

/// İşletim sisteminin rastgele kaynağından `n` bayt, küçük harf onaltılık.
pub(crate) fn random_hex(n: usize) -> String {
    let mut buf = vec![0u8; n];
    if let Err(e) = getrandom::getrandom(&mut buf) {
        // Rastgele kaynak yoksa güvenli UUID/boundary üretilemez; sessizce zayıf değer üretmek yerine durulur.
        panic!("pushizo: rastgele sayı üretilemedi: {e}");
    }
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alan_ve_dosya_kodlanir() {
        let mut mp = Multipart::new();
        mp.field("type", "users");
        mp.file("file", &FileUpload::new("a.csv", "text/csv", b"external_id\nu1\n".to_vec()));
        let (body, content_type) = mp.encode();
        let text = String::from_utf8(body).unwrap();
        let boundary = content_type.strip_prefix("multipart/form-data; boundary=").unwrap();
        assert!(text.starts_with(&format!("--{boundary}\r\n")));
        assert!(text.contains("name=\"type\"\r\n\r\nusers\r\n"));
        assert!(text.contains("filename=\"a.csv\"\r\nContent-Type: text/csv\r\n\r\nexternal_id\nu1\n\r\n"));
        assert!(text.ends_with(&format!("--{boundary}--\r\n")));
    }
}
