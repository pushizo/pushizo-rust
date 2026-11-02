//! `data` / `next_cursor` / `has_more` zarfını sayfa sayfa dolaşan yineleyici.

use crate::error::Result;

type Fetch<'a, T> = Box<dyn FnMut(Option<String>) -> Result<(Vec<T>, Option<String>, bool)> + 'a>;

/// Cursor sayfalamalı listelerin tüm öğelerini sırayla veren yineleyici (`…_all` metotları döndürür).
///
/// Sayfalar ihtiyaç oldukça istenir. Hata olursa yineleyici bir kez `Err` verir ve durur.
pub struct Paginator<'a, T> {
    fetch: Fetch<'a, T>,
    buffer: std::vec::IntoIter<T>,
    cursor: Option<String>,
    done: bool,
}

impl<'a, T> Paginator<'a, T> {
    pub(crate) fn new<F>(fetch: F) -> Self
    where
        F: FnMut(Option<String>) -> Result<(Vec<T>, Option<String>, bool)> + 'a,
    {
        Self { fetch: Box::new(fetch), buffer: Vec::new().into_iter(), cursor: None, done: false }
    }
}

impl<'a, T> std::fmt::Debug for Paginator<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Paginator").field("done", &self.done).finish_non_exhaustive()
    }
}

impl<'a, T> Iterator for Paginator<'a, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.buffer.next() {
                return Some(Ok(item));
            }
            if self.done {
                return None;
            }
            match (self.fetch)(self.cursor.take()) {
                Ok((items, next, more)) => {
                    self.buffer = items.into_iter();
                    match next.filter(|c| !c.is_empty()) {
                        Some(c) if more => self.cursor = Some(c),
                        _ => self.done = true,
                    }
                }
                Err(e) => {
                    self.done = true;
                    return Some(Err(e));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn sayfalari_sirayla_dolasir() {
        let mut calls = Vec::new();
        let items: Vec<i32> = Paginator::new(|cursor: Option<String>| {
            calls.push(cursor.clone());
            Ok(match cursor.as_deref() {
                None => (vec![1, 2], Some("c2".to_string()), true),
                Some("c2") => (vec![3], Some(String::new()), false),
                _ => (vec![], None, false),
            })
        })
        .map(|r| r.unwrap())
        .collect();
        assert_eq!(items, vec![1, 2, 3]);
        assert_eq!(calls, vec![None, Some("c2".to_string())]);
    }

    #[test]
    fn hatada_durur() {
        let mut p: Paginator<'_, i32> = Paginator::new(|_| Err(Error::network("x")));
        assert!(matches!(p.next(), Some(Err(_))));
        assert!(p.next().is_none());
    }
}
